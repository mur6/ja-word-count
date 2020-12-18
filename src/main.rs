use std::env;
use std::fs;

use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;

fn count_words(tokenizer: & mut Tokenizer, line: &str) -> usize {
    let tokens = tokenizer.tokenize(line);
    tokens.len()
}

fn main() {
    let mut tokenizer = Tokenizer::new(Mode::Normal, "");

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    //println!("My path is {}.", args[0]);
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).unwrap();
    println!("Contents opened.");
    //let mut count = 0;
    let count: usize = contents.lines() .map(|line| count_words(&mut tokenizer, line)).sum();
    println!("{}", count);
}






// fn main() {
//     let dictionary = Dictionary::setup(None, None).unwrap();
//     let tokenizer = dictionary.create();

//     let args: Vec<String> = env::args().collect();
//     let filename = &args[1];
//     //println!("My path is {}.", args[0]);
//     println!("In file {}", filename);
//     let contents = fs::read_to_string(filename).unwrap();

//     let mut count = 0;
//     for line in contents.lines() {
//         for contents in line.split_whitespace() {
//             for m in tokenizer.tokenize(contents, &Some(SplitMode::C), None).unwrap() {
//                 //println!("{}", m.surface());
//                 count += 1;
//             };
//         }
//     }
//     println!("{}", count);
// }
