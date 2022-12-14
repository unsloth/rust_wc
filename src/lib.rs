use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

    #[arg(short, long, default_value_t = false, help = "Output line count")]
    lines: bool,

    #[arg(short, long, default_value_t = false, help = "Output word count")]
    words: bool,

    #[arg(
        short = 'c',
        long,
        default_value_t = false,
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

#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn run() -> MyResult<()> {
    let mut cli = Cli::parse();

    let mut total_result = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };

    if !cli.lines && !cli.bytes && !cli.words && !cli.chars {
        cli.lines = true;
        cli.bytes = true;
        cli.words = true;
    }

    for filename in &cli.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file_data) => {
                let result = count(file_data)?;

                print_result(&cli, &result);
                // if the input was stdin, don't include a filename
                if filename != "-" {
                    print!(" {}", filename);
                }
                println!();

                total_result.num_lines += result.num_lines;
                total_result.num_words += result.num_words;
                total_result.num_bytes += result.num_bytes;
                total_result.num_chars += result.num_chars;
            }
        }
    }
    if cli.files.len() > 1 {
        print_result(&cli, &total_result);
        println!(" total");
    }
    Ok(())
}

fn print_result(cli: &Cli, result: &FileInfo) {
    if cli.lines {
        print!("{:>8}", result.num_lines)
    }
    if cli.words {
        print!("{:>8}", result.num_words)
    }
    if cli.bytes {
        print!("{:>8}", result.num_bytes)
    }
    if cli.chars {
        print!("{:>8}", result.num_chars)
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines: usize = 0;
    let mut num_words: usize = 0;
    let mut num_bytes: usize = 0;
    let mut num_chars: usize = 0;

    // start bytes_in_line as >0 to initiate the while loop
    let mut bytes_in_line = 1;
    while bytes_in_line != 0 {
        let mut buf = String::new();
        // bytes_in_line will be 0 at EOF
        bytes_in_line = file.read_line(&mut buf)?;
        if bytes_in_line != 0 {
            num_lines += 1
        };
        num_words += buf.split_whitespace().count();
        num_bytes += bytes_in_line;
        num_chars += buf.chars().count();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
