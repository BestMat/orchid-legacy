// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.
use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: &'static str,
}

#[derive(Debug)]
pub struct Lexer <'x> {
    chars: Chars<'x>,
    chars_vec: Vec<char>,
    ch: char,
    cursor: usize,
    len: usize,
}

#[derive(Debug)]
pub enum TokenKind {
    // Literals:
    Number,         // 21
    String,         // "Hello World"
    Float,          // 3.14159
    Identifier,     // println
    
    // Symbols:
    OpenParen,      // (
    CloseParen,     // )
    OpenBracket,    // [
    CloseBracket,   // ]
    OpenBrace,      // {
    CloseBrace,     // }
    Semicolon,      // ;
    Colon,          // :
    Comma,          // ,
    LessOperator,   // <
    GreaterOperator,// >
    QuestionMark,   // ?
    Pipe,           // |
    Bang,           // !
    BackSlash,      // \
    Comment,        // //
    Ampersand,      // &
    At,             // @
    Squiggly,       // ~
    Hash,           // #
    Dollar,         // $
    Quote,          // '
    DoubleQuote,    // "
    Assignment,     // =
    Equals,         // ==
    LessEquals,     // <=
    GreaterEquals,  // >=
    ThinArrow,      // ->
    FatArrow,       // =>
    
    // Arithmetic:
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Exponent,       // ^
    
    // Keywords:
    KeywordLet,     // let
    KeywordConst,   // const
    KeywordFn,      // fn
    KeywordStruct,  // struct
    KeywordEnum,    // enum
    KeywordTypedef, // typedef
    KeywordImpl,    // impl
    KeywordAsync,   // async
    KeywordAwait,   // await
    KeywordIf,      // if
    KeywordElse,    // else
    KeywordWhile,   // while
    KeywordFor,     // for
    KeywordIn,      // in
    KeywordAs,      // as
    KeywordBreak,   // break
    KeywordContinue,// continue
    KeywordTypeof,  // typeof
    KeywordReturn,  // return
    
    // Type:
    TypeN8,         // number
    TypeN16,        // number
    TypeN32,        // number
    TypeN64,        // number
    TypeN128,       // number
    TypeU8,         // unsigned
    TypeU16,        // unsigned
    TypeU32,        // unsigned
    TypeU64,        // unsigned
    TypeU128,       // unsigned
    TypeStr,        // string
    TypeString,     // string
    TypeArray,      // array
    TypeVector,     // vector
    TypeBool,       // bool
    TypeNull,       // null
    TypePtr,        // ptr
    TypeVoid,       // void
    
    // EOF:
    EOF             // EOF
}

impl <'x> Lexer <'x> {
    pub fn new(src: &'x str) -> Self {
        return Self {
            chars: src.chars(),
            chars_vec: Vec::new(),
            ch: '_',
            cursor: 0,
            len: 0,
        };
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        
        self.chars_vec = self.chars.clone().collect();
        self.len = self.chars.clone().count();
        
        while let Some(token) = self.next() {
            tokens.push(token);
        }
        
        return tokens;
    }
    
    pub fn next(&mut self) -> Option<Token> {
        let mut next_char = self.eat();
        
        match next_char {
            // Symbols:
            Some('(') => Some(Token { kind: TokenKind::OpenParen, value: "(" }),
            Some(')') => Some(Token { kind: TokenKind::CloseParen, value: ")" }),
            Some('[') => Some(Token { kind: TokenKind::OpenBracket, value: "[" }),
            Some(']') => Some(Token { kind: TokenKind::CloseBracket, value: "]" }),
            Some('{') => Some(Token { kind: TokenKind::OpenBrace, value: "{" }),
            Some('}') => Some(Token { kind: TokenKind::CloseBrace, value: "}" }),
            Some(';') => Some(Token { kind: TokenKind::Semicolon, value: ";" }),
            Some(':') => Some(Token { kind: TokenKind::Colon, value: ":" }),
            Some(',') => Some(Token { kind: TokenKind::Comma, value: "," }),
            Some('<') => {
                if self.peek() == '=' {
                    return Some(Token { kind: TokenKind::LessEquals, value: "<=" });
                } else {
                    return Some(Token { kind: TokenKind::LessOperator, value: "<" });
                }
            },
            Some('>') => {
                if self.peek() == '=' {
                    return Some(Token { kind: TokenKind::GreaterEquals, value: "<=" });
                } else {
                    return Some(Token { kind: TokenKind::GreaterOperator, value: "<" });
                }
            },
            Some('?') => Some(Token { kind: TokenKind::QuestionMark, value: "?" }),
            Some('|') => Some(Token { kind: TokenKind::Pipe, value: "|" }),
            Some('!') => Some(Token { kind: TokenKind::Bang, value: "!" }),
            Some('\\') => Some(Token { kind: TokenKind::BackSlash, value: "\\" }),
            Some('/') => {
                if self.peek() == '/' {
                    return Some(Token { kind: TokenKind::Comment, value: "//" });
                } else {
                return Some(Token { kind: TokenKind::Slash, value: "/" });
                }
            },
            Some('&') => Some(Token { kind: TokenKind::Ampersand, value: "&" }),
            Some('@') => Some(Token { kind: TokenKind::At, value: "@" }),
            Some('~') => Some(Token { kind: TokenKind::Squiggly, value: "~" }),
            Some('#') => Some(Token { kind: TokenKind::Hash, value: "#" }),
            Some('$') => Some(Token { kind: TokenKind::Dollar, value: "$" }),
            Some('\'') => Some(Token { kind: TokenKind::Quote, value: "'" }),
            Some('"') => Some(Token { kind: TokenKind::DoubleQuote, value: "\"" }),
            Some('=') => {
                if self.peek() == '=' {
                    return Some(Token { kind: TokenKind::Equals, value: "==" });
                } else if self.peek() == '>' {
                    return Some(Token { kind: TokenKind::FatArrow, value: "=>" });
                } else {
                    return Some(Token { kind: TokenKind::Assignment, value: "=" });
                }
            },
            Some('-') => {
                if self.peek() == '>' {
                    return Some(Token { kind: TokenKind::ThinArrow, value: "->" });
                } else {
                    return Some(Token { kind: TokenKind::Minus, value: "-" });
                }
            },
            
            // Other Arithmetic: (+, *, %, ^)
            Some('+') => Some(Token { kind: TokenKind::Plus, value: "+" }),
            Some('*') => Some(Token { kind: TokenKind::Star, value: "*" }),
            Some('%') => Some(Token { kind: TokenKind::Percent, value: "%" }),
            Some('^') => Some(Token { kind: TokenKind::Exponent, value: "^" }),
            Some('a'..='z' | 'A' ..= 'Z' | '_') => {
                todo!();
            }
            _ => {
                self.cursor -= 1;
                self.ch = self.chars.next_back()?;
                return None;
            }
        }
    }
    
    pub fn peek(&self) -> char {
        return self.chars.clone().next().unwrap();
    }
    
    pub fn eat(&mut self) -> Option<char> {
        self.cursor += 1;
        self.ch = self.chars.next()?;
        return Some(self.ch);
    }
    
    pub fn print_tokens(&mut self) {
        println!("{:#?}", self.tokenize());
    }
}