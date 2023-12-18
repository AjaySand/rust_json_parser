use std::{collections::BTreeMap, process};
use tokenizer::{tokenize_json, Token};

pub mod tokenizer;

pub fn parse(input: &str) {
    let tokens = tokenize_json(input);
    let value = BTreeMap::<String, String>::new();

    if !validate(&tokens) {
        eprintln!("{}", "Invalid JSON");
        process::exit(1);
    }

    let mut tokens = tokens.iter().peekable();
    while let Some(token) = tokens.peek() {
        match token {
            Token::String(_) => {
                //
            }
            Token::LeftCurlyBracket => {}
            Token::RightCurlyBracket => {}
            Token::LeftSquareBracket => {}
            Token::RightSquareBracket => {}
            _ => {}
        }

        tokens.next();
    }

    println!("{:?}", tokens);
}

fn validate(tokens: &Vec<tokenizer::Token>) -> bool {
    let mut stack: Vec<tokenizer::Token> = Vec::new();
    let mut is_valid = true;

    for token in tokens {
        match token {
            Token::LeftCurlyBracket => stack.push(Token::LeftCurlyBracket),
            Token::LeftSquareBracket => stack.push(Token::LeftSquareBracket),
            Token::RightCurlyBracket => {
                if let Some(Token::LeftCurlyBracket) = stack.pop() {
                    continue;
                } else {
                    is_valid = false;
                    break;
                }
            }
            Token::RightSquareBracket => {
                if let Some(Token::LeftSquareBracket) = stack.pop() {
                    continue;
                } else {
                    is_valid = false;
                    break;
                }
            }
            _ => continue,
        }
    }

    is_valid
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
