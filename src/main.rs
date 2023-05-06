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
        required_unless_present = "bg"
    )]
    fg: Option<ColorEnum>,

    #[clap(
        short,
        long,
        value_enum,
        value_name = "COLOR",
        required_unless_present = "fg"
    )]
    bg: Option<ColorEnum>,

    #[clap(short, long)]
    ignore_case: bool,
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
            let mut colorspec = ColorSpec::new();

            let fg = args.fg.map(|c| to_color(&c));
            let bg = args.bg.map(|c| to_color(&c));

            if fg.is_some() {
                colorspec.set_fg(fg);
            }
            if bg.is_some() {
                colorspec.set_bg(bg);
            }
            stdout.set_color(&colorspec).unwrap();
            println!("{}", &line);
            stdout.reset().unwrap();
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
