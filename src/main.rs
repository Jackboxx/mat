use std::{
    env, fs,
    io::{self, BufRead, BufReader},
};

// stuff I want to have
// -r: reverse lines
// -R: reverse text
// -j: join lines

fn main() {
    print_output(read_input());
}

fn print_output(lines: Vec<String>) {
    lines.iter().for_each(|l| println!("{l}"))
}

fn read_input() -> Vec<String> {
    match env::args().skip(1).next() {
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
