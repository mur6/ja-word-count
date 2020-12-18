use sudachiclone::prelude::*;

fn main() {
    let dictionary = Dictionary::setup(None, None).unwrap();
    let tokenizer = dictionary.create();

    // Multi-granular tokenization
    // using `system_core.dic` or `system_full.dic` version 20190781
    // you may not be able to replicate this particular example due to dictionary you use

    for m in tokenizer.tokenize("国家公務員", &Some(SplitMode::C), None).unwrap() {
        println!("{}", m.surface());
    };
    //# => 国家公務員
    //println!("Hello, world!");
}
