mod args;
mod helpers;

use args::Cli;
use clap::Parser;
use helpers::print_header;

fn main() {
    let args = Cli::parse();
    print_header();
}
