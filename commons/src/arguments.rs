use std::borrow::Cow;
use std::path::PathBuf;

/// Parse the advent of code arguments (not using clap to learn how this can be done)
pub fn parse_arguments(name: &str) -> Arguments {
    let mut args = std::env::args().skip(1);
    let mut day: Option<Day> = None;
    let mut input: Option<PathBuf> = None;
    loop {
        match next_opt(&mut args) {
            Ok(Some(Opt::Day(d))) => day = Some(d),
            Ok(Some(Opt::Input(i))) => input = Some(i),
            Ok(Some(Opt::Help)) => print_help_and_exit(name),
            Ok(None) => match (day, input) {
                (Some(day), Some(input)) => return Arguments { day, input },
                (None, _) => {
                    println!("'day' is required\n");
                    print_help_and_exit(name)
                }
                (_, None) => {
                    println!("'input' is required\n");
                    print_help_and_exit(name)
                }
            },
            Err(reason) => {
                println!("{reason}\n");
                print_help_and_exit(name);
            }
        }
    }
}

/// The Advent of Code arguments
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Arguments {
    /// The specific day of the problem
    pub day: Day,
    /// The input for that day problem
    pub input: PathBuf,
}

/// The Day of the Advent of Code problem to solve (between 01 and 25 or all)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Day {
    All,
    Number(u8),
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All days"),
            Self::Number(n) => write!(f, "Day {n}"),
        }
    }
}

enum Opt {
    Day(Day),
    Input(PathBuf),
    Help,
}

fn print_help_and_exit(name: &str) -> ! {
    println!(
        "Solutions for the advent of code problems
  Usage: {name}.exe --day <DAY> --input <FILE>
  Options:
  -d, --day <DAY>     The specific day of the problem or 'all'
  -i, --input <FILE>  The problem's input. If day is 'all', a directory from 01..txt to 25.txt
  -h, --help          Print help"
    );
    std::process::exit(1)
}

fn next_opt<Args: Iterator<Item = String>>(args: &mut Args) -> Result<Option<Opt>, String> {
    let arg = match args.next() {
        Some(a) => a,
        None => return Ok(None),
    };

    if let Some(d) = opt_value(&arg, "-d", "--day", args)? {
        if d == "all" {
            Ok(Some(Opt::Day(Day::All)))
        } else {
            match d.parse::<u8>() {
                Ok(d) if (1..26).contains(&d) => Ok(Some(Opt::Day(Day::Number(d)))),
                _ => Err(format!("day must be 'all' or a number from 1 to 25: {d}")),
            }
        }
    } else if let Some(input) = opt_value(&arg, "-i", "--input", args)? {
        Ok(Some(Opt::Input(PathBuf::from(input.into_owned()))))
    } else if arg == "-h" || arg == "--help" {
        Ok(Some(Opt::Help))
    } else {
        Err(format!("unknown argument: {arg}"))
    }
}

fn opt_value<'a, Args: Iterator<Item = String>>(
    arg: &'a String,
    short: &str,
    long: &str,
    remaining_arguments: &mut Args,
) -> Result<Option<Cow<'a, str>>, String> {
    // Short arguments can contain the value directly after the prefix
    // Long arguments can contain the value after the prefix separated by a '='
    // Some if the argument matches the opt, the inner option contains the value if present
    let value = arg
        .strip_prefix(short)
        .map(Some)
        .or_else(|| arg.strip_prefix(long).map(|r| r.strip_prefix('=')));

    match value {
        Some(Some(v)) if !v.is_empty() => Ok(Some(Cow::Borrowed(v))),
        // If the value was not directly inside the argument, find it from the remaining args
        Some(_) => match remaining_arguments.next() {
            Some(v) => Ok(Some(Cow::Owned(v))),
            None => Err(format!("missing value for argument {arg}")),
        },
        None => Ok(None),
    }
}
