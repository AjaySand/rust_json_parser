use std::iter::Iterator;
use std::{collections::BTreeMap, process};

pub mod tokenizer;
use tokenizer::{tokenize_json, Token};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsonValue {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

enum ConsoleColors {
    RED(bool),
    GREEN(bool),
    YELLOW(bool),
    BLUE(bool),
    WHITE(bool),
    None,
}

fn print_colorize_string_for_console(str: String, color: ConsoleColors) {
    const RED: i16 = 31;
    const GREEN: i16 = 32;
    const YELLOW: i16 = 33;
    const BLUE: i16 = 34;
    const WHITE: i16 = 97;

    let s = match color {
        ConsoleColors::RED(true) => format!("\x1b[{}m{}\x1b[0m", RED, str),
        ConsoleColors::GREEN(true) => format!("\x1b[{}m{}\x1b[0m", GREEN, str),
        ConsoleColors::YELLOW(true) => format!("\x1b[{}m{}\x1b[0m", YELLOW, str),
        ConsoleColors::BLUE(true) => format!("\x1b[{}m{}\x1b[0m", BLUE, str),
        ConsoleColors::WHITE(true) => format!("\x1b[{}m{}\x1b[0m", WHITE, str),
        _ => str,
    };

    println!("{}", s);
}

fn get_next_token<'a, I>(tokens: &mut I) -> &'a Token
where
    I: Iterator<Item = &'a Token> + Clone,
{
    let token = match tokens.next() {
        Some(token) => token,
        None => {
            panic!("Unexpected value found");
        }
    };

    token.to_owned()
}

// https://github.com/eatonphil/pj/blob/master/pj/parser.py
fn parse_list<'a, I>(parent_value: &mut Vec<JsonValue>, tokens: &mut I)
where
    I: Iterator<Item = &'a Token> + Clone,
{
    let first_token = tokens.clone().peekable().peek().unwrap().to_owned();
    if first_token == &Token::RightSquareBracket {
        // this is and empty list

        print_colorize_string_for_console(
            format!("{:?}", parent_value),
            ConsoleColors::YELLOW(true),
        );
        return;
    }

    loop {
        let value = get_next_token(tokens);
        match value {
            // string
            Token::String(val) => {
                print_colorize_string_for_console(
                    format!("string {:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.push(JsonValue::String(val.into()));
            }
            // number
            Token::Number(val) => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.push(JsonValue::Number(val.into()));
            }
            // object
            Token::LeftCurlyBracket => {
                let mut val = BTreeMap::<String, JsonValue>::new();

                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::YELLOW(true),
                );
                parse_object(&mut val, tokens);

                parent_value.push(JsonValue::Object(val));
            }
            // array
            Token::LeftSquareBracket => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::YELLOW(true),
                );

                let mut value = Vec::<JsonValue>::new();
                parse_list(&mut value, tokens);

                parent_value.push(JsonValue::Array(value));
            }
            // true
            Token::True => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.push(JsonValue::Boolean(true))
            }
            // false
            Token::False => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.push(JsonValue::Boolean(false));
            }
            // null
            Token::Null => {
                print_colorize_string_for_console(
                    format!("{:?} (null)", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.push(JsonValue::Null);
            }
            _ => {}
        }

        let token = get_next_token(tokens);
        if let Token::Comma = token {
            // print_colorize_string_for_console(format!("{:?}", token), ConsoleColors::RED(true));
        } else if let Token::RightSquareBracket = token {
            print_colorize_string_for_console(format!("{:?}", token), ConsoleColors::YELLOW(true));
            break;
        } else {
            panic!("Unexpected value found. Expected comma found {:?}", token);
        }
        print!("\n");
    }
}

fn parse_object<'a, I>(parent_value: &mut BTreeMap<String, JsonValue>, tokens: &mut I)
where
    I: Iterator<Item = &'a Token> + Clone,
{
    let first_token = tokens.clone().peekable().peek().unwrap().to_owned();
    if first_token == &Token::RightCurlyBracket {
        // this is and empty object
        // todo: figure out hwo to hadle it

        print_colorize_string_for_console(format!("{:?}", parent_value), ConsoleColors::BLUE(true));
        return;
    }

    loop {
        // get key
        let token = get_next_token(tokens);
        if let Token::String(key) = token {
            print_colorize_string_for_console(format!("{:?}", key), ConsoleColors::BLUE(true));
        } else {
            panic!("Unexpected value found. Expected string found {:?}", token);
        }

        // get colon
        let colon_token = get_next_token(tokens);
        if let Token::Colon = colon_token {
            // print_colorize_string_for_console(format!("{:?}", token), ConsoleColors::RED(true));
        } else {
            panic!(
                "Unexpected value found. Expected colon found {:?}",
                colon_token
            );
        }

        // this is the name in name:value pair
        let Token::String(name) = &token else { todo!() };

        // get value
        let value = get_next_token(tokens);
        match value {
            Token::String(val) => {
                print_colorize_string_for_console(
                    format!("string {:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.insert(name.into(), JsonValue::String(val.into()));
            }
            Token::Number(val) => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.insert(name.into(), JsonValue::String(val.into()));
            }
            Token::True => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.insert(name.into(), JsonValue::Boolean(true));
            }
            Token::False => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.insert(name.into(), JsonValue::Boolean(false));
            }
            Token::Null => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::GREEN(true),
                );

                parent_value.insert(name.into(), JsonValue::Null);
            }
            Token::LeftSquareBracket => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::YELLOW(true),
                );

                let mut value = Vec::<JsonValue>::new();
                parse_list(&mut value, tokens);

                parent_value.insert(name.into(), JsonValue::Array(value));
            }
            Token::LeftCurlyBracket => {
                let mut val = BTreeMap::<String, JsonValue>::new();

                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::YELLOW(true),
                );
                parse_object(&mut val, tokens);

                parent_value.insert(name.into(), JsonValue::Object(val));
            }
            _ => {
                print_colorize_string_for_console(
                    format!("{:?}", value),
                    ConsoleColors::WHITE(true),
                );
            }
        }

        let token = get_next_token(tokens);
        if let Token::Comma = token {
            // print_colorize_string_for_console(format!("{:?}", token), ConsoleColors::RED(true));
        } else if let Token::RightCurlyBracket = token {
            print_colorize_string_for_console(format!("{:?}", token), ConsoleColors::YELLOW(true));
            break;
        } else {
            panic!("Unexpected value found. Expected comma found {:?}", token);
        }
        print!("\n");
    }
}

pub fn parse(input: &str) -> BTreeMap<String, JsonValue> {
    let tokens = tokenize_json(input);
    let mut value = BTreeMap::<String, JsonValue>::new();

    if !validate(&tokens) {
        eprintln!("{}", "Invalid JSON");
        process::exit(1);
    }

    let mut tokens = tokens.iter();

    let p = tokens.next().unwrap();
    if p == &Token::LeftCurlyBracket {
        parse_object(&mut value, &mut tokens);
    }

    value
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
