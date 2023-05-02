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
#[derive(Debug, Clone)]
pub struct Arguments {
    /// The specific day of the problem
    pub day: Day,
    /// The input for that day problem
    pub input: PathBuf,
}

/// The Day of the Advent of Code problem to solve (between 01 and 25 or all)
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Day {
    All = 0,
    Day1 = 1,
    Day2 = 2,
    Day3 = 3,
    Day4 = 4,
    Day5 = 5,
    Day6 = 6,
    Day7 = 7,
    Day8 = 8,
    Day9 = 9,
    Day10 = 10,
    Day11 = 11,
    Day12 = 12,
    Day13 = 13,
    Day14 = 14,
    Day15 = 15,
    Day16 = 16,
    Day17 = 17,
    Day18 = 18,
    Day19 = 19,
    Day20 = 20,
    Day21 = 21,
    Day22 = 22,
    Day23 = 23,
    Day24 = 24,
    Day25 = 25,
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
        let day = match d.parse().unwrap_or(0u8) {
            0 if d == "all" => Day::All,
            1 => Day::Day1,
            2 => Day::Day2,
            3 => Day::Day3,
            4 => Day::Day4,
            5 => Day::Day5,
            6 => Day::Day6,
            7 => Day::Day7,
            8 => Day::Day8,
            9 => Day::Day9,
            10 => Day::Day10,
            11 => Day::Day11,
            12 => Day::Day12,
            13 => Day::Day13,
            14 => Day::Day14,
            15 => Day::Day15,
            16 => Day::Day16,
            17 => Day::Day17,
            18 => Day::Day18,
            19 => Day::Day19,
            20 => Day::Day20,
            21 => Day::Day21,
            22 => Day::Day22,
            23 => Day::Day23,
            24 => Day::Day24,
            25 => Day::Day25,
            _ => return Err(format!("day must be 'all' or a number from 1 to 25: {d}")),
        };
        Ok(Some(Opt::Day(day)))
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
