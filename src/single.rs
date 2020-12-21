use std::env;
use std::fs;

use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;

fn count_words(tokenizer: &mut Tokenizer, line: &str) -> usize {
    let tokens = tokenizer.tokenize(line);
    tokens.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap_or_else(|| {
        println!("Please give the input file.");
        std::process::exit(1);
    });
    let mut tokenizer = Tokenizer::new(Mode::Normal, "");
    let contents = fs::read_to_string(filename).unwrap();
    let count: usize = contents
        .lines()
        .map(|line| count_words(&mut tokenizer, line),
        )
        .sum();
    println!(" {} {}", count, filename);
}
