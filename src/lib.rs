use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
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

    #[arg(short, long, help = "Output line count")]
    lines: bool,

    #[arg(short, long, help = "Output word count")]
    words: bool,

    #[arg(short = 'c', long, help = "Output number of bytes")]
    bytes: bool,

    #[arg(short = 'm', long, help = "Output number of chars")]
    chars: bool,
}

pub fn run() -> MyResult<()> {
    let cli = Cli::parse();
    Ok(())
}
