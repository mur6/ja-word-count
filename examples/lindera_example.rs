use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;

fn main() {
    let mut tokenizer = Tokenizer::new(Mode::Normal, "");
    let tokens = tokenizer.tokenize("Rustは難しいが、面白い。");
    for token in tokens {
        println!("{}\t{:?}", token.text, token.detail);
    }
}
