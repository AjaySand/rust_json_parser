use rust_json_parser::tokenizer;

fn main() {
    let input = r#"
        {
            "Image": {
                "Width":  800,
                "Height": 600,
                "Title":  "View from 15th Floor",
                "Thumbnail": {
                    "Url":    "http://www.example.com/image/481989943",
                    "asdf":    "",
                    "Height": 125,
                    "Width":  100
                },
                "Animated" : false,
                "IDs": [116, 943, 234, 38793]
            }
        }
    "#;

    let tokens = tokenizer::tokenize_json(input);
    tokens.into_iter().for_each(|token| println!("{:?}", token));
}
