use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Print newline, word, and byte counts for each FILE, 
and a total line if more than one FILE is specified.
With no FILE, or when FILE is -, read standard input."
)]
pub struct Cli {
    #[arg(default_values = vec!["-"], help = "File(s) to count")]
    files: Vec<String>,

    #[arg(short, long, default_value_t = true, help = "Output line count")]
    lines: bool,

    #[arg(short, long, default_value_t = true, help = "Output word count")]
    words: bool,

    #[arg(
        short = 'c',
        long,
        default_value_t = true,
        help = "Output number of bytes"
    )]
    bytes: bool,

    #[arg(
        short = 'm',
        long,
        default_value_t = false,
        conflicts_with = "bytes",
        help = "Output number of chars"
    )]
    chars: bool,
}

pub fn run() -> MyResult<()> {
    let cli = Cli::parse();
    println!("{:#?}", cli);
    Ok(())
}
