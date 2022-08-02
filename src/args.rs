use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
pub struct Args {
    #[clap(
        short = 'c',
        long = "cols",
        help = "Set the number of colums for the connect 4 board",
        default_value = "7"
    )]
    pub cols: usize,

    #[clap(
        short = 'r',
        long = "rows",
        help = "Set the number of rows for the connect 4 board",
        default_value = "6"
    )]
    pub rows: usize,
}
