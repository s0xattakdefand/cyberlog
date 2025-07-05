use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cyberlog")]
#[command(about = "Red/Blue Team Daily Journal CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        #[arg(long)]
        role: String,
    },
    List,
    View {
        #[arg(long)]
        id: i32,
    },
    Delete {
        #[arg(long)]
        id: i32,
    },
    Edit {
        #[arg(long)]
        id: i32,
    },
    Search {
        #[arg(long)]
        field: String,
        #[arg(long)]
        keyword: String,
    },
}
