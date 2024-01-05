use rust_json_parser::{parse, tokenizer::tokenize_json};

fn main() {
    let input = include_str!("../example/inputs/test_1.json");

    let obj = parse(input);
    println!("{:#?}", obj);
}
