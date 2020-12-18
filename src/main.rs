use std::env;
use std::fs;

use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;
use rayon::prelude::*;

fn count_words(tokenizer: &mut Tokenizer, line: &str) -> usize {
    let tokens = tokenizer.tokenize(line);
    tokens.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let count: usize = contents
        .par_lines()
        .map_init(
            || Tokenizer::new(Mode::Normal, ""),
            |tokenizer, line| count_words(tokenizer, line),
        )
        .sum();
    println!(" {} {}", count, filename);
}
