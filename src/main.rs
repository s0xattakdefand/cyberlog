mod cli;
mod db;
mod journal;

use clap::Parser;
use cli::Cli;
use db::init_db;
use journal::{
    handle_new, handle_list, handle_view, handle_delete, handle_edit, handle_search,
};

fn main() {
    let cli = Cli::parse();
    init_db().expect("Failed to initialize database");

    match cli.command {
        cli::Commands::New { role } => handle_new(role).unwrap(),
        cli::Commands::List => handle_list().unwrap(),
        cli::Commands::View { id } => handle_view(id).unwrap(),
        cli::Commands::Delete { id } => handle_delete(id).unwrap(),
        cli::Commands::Edit { id } => handle_edit(id).unwrap(),
        cli::Commands::Search { field, keyword } => handle_search(field, keyword).unwrap(),
    }
}
