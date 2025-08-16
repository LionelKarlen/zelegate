use std::path::PathBuf;

use crate::{
    config::{ZelegateConfig, read_config},
    into::into,
    list::show_list,
};
use clap::{Parser, Subcommand};

mod config;
mod into;
mod list;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Arguments {
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Into { destination: String },
    Dump,
}

fn main() {
    let arguments = Arguments::parse();

    let config_path = match arguments.config {
        Some(s) => s,
        None => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("zelegate");
            xdg_dirs
                .get_config_file("zelegate.toml")
                .expect("This shouldn't happen. Does $HOME exist?")
        }
    };
    let config: ZelegateConfig = read_config(config_path);

    match &arguments.command {
        Commands::List => {
            show_list(&config);
        }
        Commands::Into { destination } => {
            into(destination, &config);
        }
        Commands::Dump => {
            // TODO: Clean up (serialise)
            println!("{:?}", config);
        }
    }
}
