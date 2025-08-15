use crate::list::show_list;
use clap::{Parser, Subcommand};

mod list;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Arguments {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List,
}

fn main() {
    let arguments = Arguments::parse();

    match &arguments.command {
        Some(Commands::List) => {
            show_list();
        }
        None => {}
    }
}
