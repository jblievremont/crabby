mod greetings;
mod chifoumi;
mod articles;

use greetings::greets;
use chifoumi::{ play, Game };
use articles::search_articles;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    author = "JBL",
    version = "1.0.0",
    about = "Crabby",
    long_about = None
)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greets with name
    Greets {
        /// Name of the person to greet
        #[clap(short, long, value_parser)]
        name: String,
    },
    /// chifoumi with players
    Chifoumi {
        #[clap(short = 'a', long, value_enum)]
        one: Game,
        /// random game if not provided
        #[clap(short = 'b', long, value_enum)]
        two: Option<Game>,
    },
    /// search articles on Hacker News
    Search {
        #[clap(short = 'k', long, value_parser)]
        keyword: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Chifoumi { one, two } => match two {
            // player two provided
            Some(p) => {
                let result = play(one.clone(), p.clone());
                println!("p1: {:?} vs p2: {:?} => {:?}", one, p, result)
            }
            None => {
                panic!("I don't know how to handle this")
            }
        },
        Commands::Greets { name } => {
            println!("{}", greets(name));
        },
        Commands::Search { keyword } => {
            let result = search_articles(keyword).unwrap();
            println!("Found {} results", result.nb_hits);
            for article in result.hits.iter() {
                let title = &article.title;
                println!("* {title}");
            }
        }
    }
}
