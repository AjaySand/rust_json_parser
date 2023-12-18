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
                } else {
                    // TODO: handle sytax error
                }
            }
            'f' => {
                let taken = chars.by_ref().take(5).collect::<String>();
                if taken == "false" {
                    tokens.push(Token::False);
                } else {
                    // TODO: handle sytax error
                }
            }
            'n' => {
                let taken = chars.by_ref().take(4).collect::<String>();
                if taken == "null" {
                    tokens.push(Token::False);
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
        println!("{:?}", tokens);
    }
}
