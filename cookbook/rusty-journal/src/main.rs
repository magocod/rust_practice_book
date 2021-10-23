mod cli;
mod tasks;
use structopt::StructOpt;

fn main() {
    // cargo run -- add "buy milk"
    // cargo run -- done 4
    // cargo run -- -j groceries.txt list

    // cli::CommandLineArgs::from_args();
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
