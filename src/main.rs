use clap::{Parser, ValueEnum};
use regex::Regex;
use std::io::{self, BufRead};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Dylan Kendal",
    about = "Reads lines from stdin and colorizes them based on patterns"
)]
struct Cli {
    #[clap(short, long, value_name = "PATTERN", required = true)]
    pattern: String,

    #[clap(
        short,
        long,
        value_enum,
        value_name = "COLOR",
        required_unless_present = "bg",
        help = "Set the foreground color"
    )]
    fg: Option<ColorEnum>,

    #[clap(
        short,
        long,
        value_enum,
        value_name = "COLOR",
        required_unless_present = "fg",
        help = "Set the background color"
    )]
    bg: Option<ColorEnum>,

    #[clap(short, long, help = "Ignore case distinctions in PATTERN")]
    ignore_case: bool,

    #[clap(short, long, help = "Only print the matched parts of the line")]
    only_matching: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ColorEnum {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    White,
    Magenta,
}

fn main() {
    let args = Cli::parse();

    let regex = if args.ignore_case {
        Regex::new(&format!("(?i){}", args.pattern)).unwrap()
    } else {
        Regex::new(&args.pattern).unwrap()
    };

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if regex.is_match(&line) {
            let line = strip_ansi_escapes(&line);

            let matched = regex.find(&line).unwrap();

            let colorspec = ColorSpec::new()
                .set_fg(args.fg.map_or(None, |c| Some(to_color(&c))))
                .set_bg(args.bg.map_or(None, |c| Some(to_color(&c))))
                .to_owned();

            if args.only_matching {
                let before = &line[..matched.start()];
                let after = &line[matched.end()..];
                print!("{}", before);
                stdout.set_color(&colorspec).expect("Failed to set color");
                println!("{}", matched.as_str());
                stdout.reset().expect("Failed to reset stdout");
                print!("{}", after);
            } else {
                stdout.set_color(&colorspec).expect("Failed to set color");
                println!("{}", &line);
                stdout.reset().expect("Failed to reset stdout");
            }
        } else {
            println!("{}", &line);
        }
    }
}

fn to_color(color_enum: &ColorEnum) -> Color {
    match color_enum {
        ColorEnum::Black => Color::Black,
        ColorEnum::Red => Color::Red,
        ColorEnum::Green => Color::Green,
        ColorEnum::Yellow => Color::Yellow,
        ColorEnum::Blue => Color::Blue,
        ColorEnum::Cyan => Color::Cyan,
        ColorEnum::White => Color::White,
        ColorEnum::Magenta => Color::Magenta,
    }
}

fn strip_ansi_escapes(line: &str) -> String {
    let re = Regex::new("\x1b\\[(\\d|;)*[a-zA-Z]").unwrap();
    re.replace_all(line, "").to_string()
}
