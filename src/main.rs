use crate::{into::into, list::show_list};
use clap::{Parser, Subcommand};

mod into;
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
    Into { destination: String },
}

fn main() {
    let arguments = Arguments::parse();

    match &arguments.command {
        Some(Commands::List) => {
            show_list();
        }
        Some(Commands::Into { destination }) => {
            into(destination);
        }
        None => {}
    }
}
