use clap::Parser;

mod args;
mod board;

fn main() {
    let args = args::Args::parse();
    let size = args.size;
    let board = board::Board::new(size);

    println!("{}", board);
}
