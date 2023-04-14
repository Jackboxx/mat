use std::{
    fs,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MatArgs {
    // File to read input from
    #[arg(short, long)]
    file: Option<String>,

    // Reverse lines before printing
    #[arg(short('r'), long)]
    reverse_lines: bool,

    // Reverse text in lines before printing
    #[arg(short('R'), long)]
    reverse_text: bool,

    // Join lines together
    #[arg(short, long)]
    join: bool,
}

fn main() {
    let args = MatArgs::parse();
    print_output(read_input(args.file.clone()), args);
}

fn print_output(lines: Vec<String>, args: MatArgs) {
    let mut line_iter: Box<dyn DoubleEndedIterator<Item = String>> = Box::new(lines.into_iter());

    if args.reverse_lines {
        line_iter = Box::new(line_iter.rev());
    }

    if args.reverse_text {
        line_iter = Box::new(line_iter.map(|l| l.chars().rev().collect()));
    }

    if args.join {
        println!("{}", line_iter.collect::<Vec<String>>().join(""));
    } else {
        line_iter.for_each(|l| println!("{l}"));
    }
}

fn read_input(file: Option<String>) -> Vec<String> {
    match file {
        Some(filename) => {
            let file = fs::File::open(filename).unwrap();
            BufReader::new(file).lines().map(|l| l.unwrap()).collect()
        }
        None => {
            let stdin = io::stdin();
            BufReader::new(stdin).lines().map(|l| l.unwrap()).collect()
        }
    }
}
