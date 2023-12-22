#[derive(Debug, PartialEq)]
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
*/
pub fn tokenize_json(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.peek() {
        match c {
            '[' => tokens.push(Token::LeftSquareBracket),
            ']' => tokens.push(Token::RightSquareBracket),
            '{' => tokens.push(Token::LeftCurlyBracket),
            '}' => tokens.push(Token::RightCurlyBracket),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '"' => {
                let mut string = String::new();
                chars.next(); // consume the '"' at the start of the string
                loop {
                    let c = chars.peek().unwrap();
                    if c == &'\\' {
                        chars.next();
                        let c = chars.next().unwrap();
                        string.push(c);
                        continue;
                    } else if c == &'"' {
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
                    if !n.is_digit(10) && n != &'.' {
                        break;
                    }

                    string.push(*n);
                    chars.next();
                }
                tokens.push(Token::Number(string));
                continue; // skip chars.next() at the end of the loop
            }
            't' => {
                let taken = chars.by_ref().take(4).collect::<String>();
                if taken == "true" {
                    tokens.push(Token::True);
                    continue;
                } else {
                    // TODO: handle sytax error
                }
            }
            'f' => {
                let taken = chars.by_ref().take(5).collect::<String>();
                if taken == "false" {
                    tokens.push(Token::False);
                    continue;
                } else {
                    // TODO: handle sytax error
                }
            }
            'n' => {
                let taken = chars.by_ref().take(4).collect::<String>();
                if taken == "null" {
                    tokens.push(Token::False);
                    continue;
                } else {
                    // TODO: handle sytax error
                }
            }
            _ => {
                //
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
        let input = include_str!("../example/inputs/test_1.json");

        let tokens = tokenize_json(input);
        assert_eq!(tokens.len(), 65);

        assert_eq!(tokens[0], Token::LeftCurlyBracket);
        assert_eq!(tokens[1], Token::String(String::from("Image")));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::LeftCurlyBracket);

        assert_eq!(tokens[4], Token::String(String::from("Width")));
        assert_eq!(tokens[5], Token::Colon);
        assert_eq!(tokens[6], Token::Number(String::from("800")));
        assert_eq!(tokens[7], Token::Comma);

        assert_eq!(tokens[8], Token::String(String::from("Height")));
        assert_eq!(tokens[9], Token::Colon);
        assert_eq!(tokens[10], Token::Number(String::from("600")));
        assert_eq!(tokens[11], Token::Comma);

        assert_eq!(tokens[12], Token::String(String::from("Title")));
        assert_eq!(tokens[13], Token::Colon);
        assert_eq!(
            tokens[14],
            Token::String(String::from("View from 15th Floor"))
        );
        assert_eq!(tokens[15], Token::Comma);

        assert_eq!(tokens[16], Token::String(String::from("Thumbnail")));
        assert_eq!(tokens[17], Token::Colon);
        assert_eq!(tokens[18], Token::LeftCurlyBracket);

        assert_eq!(tokens[19], Token::String(String::from("Url")));
        assert_eq!(tokens[20], Token::Colon);
        assert_eq!(
            tokens[21],
            Token::String(String::from("http://www.example.com/image/481989943"))
        );
        assert_eq!(tokens[22], Token::Comma);

        assert_eq!(tokens[23], Token::String(String::from("asdf")));
        assert_eq!(tokens[24], Token::Colon);
        assert_eq!(tokens[25], Token::String(String::from("")));
        assert_eq!(tokens[26], Token::Comma);

        assert_eq!(tokens[27], Token::String(String::from("Height")));
        assert_eq!(tokens[28], Token::Colon);
        assert_eq!(tokens[29], Token::Number(String::from("125")));
        assert_eq!(tokens[30], Token::Comma);

        assert_eq!(tokens[31], Token::String(String::from("Width")));
        assert_eq!(tokens[32], Token::Colon);
        assert_eq!(tokens[33], Token::Number(String::from("100")));
        assert_eq!(tokens[34], Token::Comma);

        assert_eq!(tokens[35], Token::String(String::from("aspectRatio")));
        assert_eq!(tokens[36], Token::Colon);
        assert_eq!(tokens[37], Token::Number(String::from("1.2")));

        assert_eq!(tokens[38], Token::RightCurlyBracket);
        assert_eq!(tokens[39], Token::Comma);

        assert_eq!(tokens[40], Token::String(String::from("Animated")));
        assert_eq!(tokens[41], Token::Colon);
        assert_eq!(tokens[42], Token::False);
        assert_eq!(tokens[43], Token::Comma);

        assert_eq!(tokens[44], Token::String(String::from("AnimateOnHover")));
        assert_eq!(tokens[45], Token::Colon);
        assert_eq!(tokens[46], Token::True);
        assert_eq!(tokens[47], Token::Comma);

        assert_eq!(tokens[48], Token::String(String::from("IDs")));
        assert_eq!(tokens[49], Token::Colon);
        assert_eq!(tokens[50], Token::LeftSquareBracket);
        assert_eq!(tokens[51], Token::Number(String::from("116")));
        assert_eq!(tokens[52], Token::Comma);

        assert_eq!(tokens[53], Token::Number(String::from("943")));
        assert_eq!(tokens[54], Token::Comma);

        assert_eq!(tokens[55], Token::Number(String::from("234")));
        assert_eq!(tokens[56], Token::Comma);

        assert_eq!(tokens[57], Token::Number(String::from("38793")));
        assert_eq!(tokens[58], Token::RightSquareBracket);
        assert_eq!(tokens[59], Token::RightCurlyBracket);
        assert_eq!(tokens[60], Token::Comma);

        assert_eq!(tokens[61], Token::String(String::from("Escaped string")));
        assert_eq!(tokens[62], Token::Colon);
        assert_eq!(
            tokens[63],
            Token::String(String::from("\\ and \" are escaped in this lifet"))
        );
    }
}
