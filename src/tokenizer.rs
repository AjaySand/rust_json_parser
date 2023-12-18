#[derive(Debug)]
pub enum Token {
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Colon,
    Comma,
    String(String),
    Number(String),
    True,
    False,
    Null,
}

/*
* TODO:
* - handle escape characters
* - while processing numbers comma (',') are not being tokenized
* - add bool (true and false) to match case
*/
pub fn tokenize_json(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.peek() {
        // if !c.is_whitespace() {
        //     println!("Parsing '{}'", c);
        // }
        match c {
            '[' => tokens.push(Token::LeftSquareBracket),
            ']' => tokens.push(Token::RightSquareBracket),
            '{' => tokens.push(Token::LeftCurlyBracket),
            '}' => tokens.push(Token::RightCurlyBracket),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '"' => {
                let mut string = String::new();
                chars.next(); // consume the first '"'
                loop {
                    // TODO: handle string escape
                    let c = chars.peek().unwrap();
                    if c == &'"' {
                        break;
                    }

                    let c = chars.next().unwrap();
                    string.push(c);
                }
                tokens.push(Token::String(string));
            }
            '0'..='9' => {
                let mut string = String::new();

                while let Some(n) = chars.peek() {
                    if !n.is_digit(10) {
                        break;
                    }

                    string.push(*n);
                    chars.next();
                }
                tokens.push(Token::Number(string));
                continue; // skip chars.next() at the end of the loop
            }
            _ => {
                if !c.is_whitespace() {
                    // println!("default case '{}'", c);
                }
            }
        }
        chars.next();
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_json_test() {
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

        let tokens = tokenize_json(input);
        println!("{:?}", tokens);
    }
}
