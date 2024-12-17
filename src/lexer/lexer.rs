// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: &'static str,
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