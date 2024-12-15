// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number,
    Identifier,
    Equals,
    
    // Symbols:
    Comma,
    Dot,
    Colon,
    Semicolon,
    OpenParam,
    CloseParam,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Quote,
    BinaryOperator,
    Ampersand,
    Star,
    
    Let,
    Const,
    
    // Types:
    N8, N16, N32, N64, N128,
    U8, U16, U32, U64, U128,
    F32, F64,
    STR,
    BOOL,
    STRING,
    ARRAY,
    VECTOR,
    POINTER,
    OBJECT,
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    value: String,
    val_type: TokenType
}

fn token(value: String, val_type: TokenType) -> Token {
    return Token { value, val_type };
}

fn is_alpha(c: char) -> bool {
    return c.is_alphabetic() || is_int(c);
}

fn is_int(c: char) -> bool {
    return c.is_digit(10);
}

fn is_skippable(str: char) -> bool {
    return str == ' ' || str == '\n' || str == '\t' || str == '\r';
}

pub fn tokenize(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut src: VecDeque<char> = code.chars().collect();
    let mut keywords = HashMap::new();
    
    keywords.insert(String::from("let"), TokenType::Let);
    keywords.insert(String::from("const"), TokenType::Const);
    keywords.insert(String::from("n8"), TokenType::N8);
    keywords.insert(String::from("n16"), TokenType::N16);
    keywords.insert(String::from("n32"), TokenType::N32);
    keywords.insert(String::from("n64"), TokenType::N64);
    keywords.insert(String::from("n128"), TokenType::N128);
    keywords.insert(String::from("u8"), TokenType::U8);
    keywords.insert(String::from("u16"), TokenType::U16);
    keywords.insert(String::from("u32"), TokenType::U32);
    keywords.insert(String::from("u64"), TokenType::U64);
    keywords.insert(String::from("u128"), TokenType::U128);
    keywords.insert(String::from("str"), TokenType::STR);
    keywords.insert(String::from("string"), TokenType::STRING);
    keywords.insert(String::from("array"), TokenType::ARRAY);
    keywords.insert(String::from("vector"), TokenType::VECTOR);
    keywords.insert(String::from("ptr"), TokenType::POINTER);
    keywords.insert(String::from("object"), TokenType::OBJECT);
    
    while !src.is_empty() {
        match src.pop_front().unwrap() {
            '(' => tokens.push(token("(".to_string(), TokenType::OpenParam)),
            ')' => tokens.push(token(")".to_string(), TokenType::CloseParam)),
            '{' => tokens.push(token("{".to_string(), TokenType::OpenBrace)),
            '}' => tokens.push(token("}".to_string(), TokenType::CloseBrace)),
            '[' => tokens.push(token("[".to_string(), TokenType::OpenBracket)),
            ']' => tokens.push(token("]".to_string(), TokenType::CloseBracket)),
            '+' => tokens.push(token("+".to_string(), TokenType::BinaryOperator)),
            '-' => tokens.push(token("-".to_string(), TokenType::BinaryOperator)),
            '*' => tokens.push(token("*".to_string(), TokenType::Star)),
            '/' => tokens.push(token("/".to_string(), TokenType::BinaryOperator)),
            '%' => tokens.push(token("%".to_string(), TokenType::BinaryOperator)),
            '&' => tokens.push(token("&".to_string(), TokenType::Ampersand)),
            '=' => tokens.push(token("=".to_string(), TokenType::Equals)),
            ';' => tokens.push(token(";".to_string(), TokenType::Semicolon)),
            ':' => tokens.push(token(":".to_string(), TokenType::Colon)),
            ',' => tokens.push(token(",".to_string(), TokenType::Comma)),
            '.' => tokens.push(token(".".to_string(), TokenType::Dot)),
            '"' => tokens.push(token("\"".to_string(), TokenType::Quote)),
            c if is_skippable(c) => {}
            c if is_int(c) => {
                let mut num = String::new();
                num.push(c);

                while !src.is_empty() && is_int(src[0]) {
                    num.push(src.pop_front().unwrap());
                }

                tokens.push(token(num, TokenType::Number));
            }
            c if is_alpha(c) => {
                let mut identifier = String::new();
                identifier.push(c);

                while !src.is_empty() && is_alpha(src[0]) {
                    identifier.push(src.pop_front().unwrap());
                }

                if let Some(token_type) = keywords.get(&identifier) {
                    tokens.push(token(identifier, token_type.clone()));
                } else {
                    tokens.push(token(identifier, TokenType::Identifier));
                }
            }
            _ => {
                panic!("BestOrchid: Unknown character found - {}", src[0]);
            }
        }
    }
    
    tokens.push(Token { val_type: TokenType::EOF, value: String::from("EndOfFile") });
    return tokens;
}