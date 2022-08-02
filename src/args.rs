use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
pub struct Args {
    #[clap(
        short = 's',
        long = "size",
        help = "Set the size of the connect 4 board",
        default_value = "4"
    )]
    pub size: usize,
}
