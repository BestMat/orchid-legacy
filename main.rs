// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.
// ---------------------------------------------------------------
// |                                                             |
// |******************* BESTORCHID - BESTMAT ********************|
// |                       Main Rust File                        |
// |                                                             |
// ---------------------------------------------------------------

use std::collections::HashMap;
use std::collections::VecDeque;
use std::env::vars;
use std::fs;    

fn main() {
    println!("I Love You Amma");

    let mut stack = Stack::new();
    stack.push(String::from("x"), Variable { val_type: Type::n8(21), is_constant: true });
    stack.push(String::from("y"), Variable { val_type: Type::n8(27), is_constant: true });
    stack.push(String::from("z"), Variable { val_type: Type::n8(0), is_constant: false });
    stack.push(String::from("a"), Variable { val_type: Type::n8(111), is_constant: false });
    stack.set(String::from("z"), Type::n8(1));
    stack.push(String::from("s"), Variable { val_type: Type::string(OrchidString::new(String::from("Hello"))), is_constant: false });
    stack.delete(String::from("a"));
    
    stack.push(String::from("ptr"), Variable { val_type: Type::pointer(OrchidPointer::new(String::from("s"))), is_constant: false });
    stack.set(String::from("s"), Type::string(OrchidString::new(String::from("Hi"))));
    stack.push(String::from("obj"), Variable { val_type: Type::object(OrchidObject::new()), is_constant: false });
    
    stack.push(String::from("println"), Variable { val_type: Type::nativefn(OrchidNativeFunction { function: stdlib_println }), is_constant: true });
    stack.call(String::from("println"), vec![Type::str(String::from("Nagapillaiyar"))]);
    
    // let ptr = stack.get(String::from("ptr")).val_type.clone();
    // if let Type::pointer(ref pointer) = ptr {
    //     let variable = pointer.get(stack);
    //     println!("{:#?}", variable.val_type);
    // }
    
    stack.clone().print();
    
    let code = String::from(fs::read_to_string("test/main.orc").unwrap());
    let mut parser = Parser::new(code);
    let ast = parser.generate_ast();
    // let mut interpreter = Interpreter::new(Expr::Program(ast.clone()));
    // interpreter.evaluate(Expr::Program(ast), Stack::new());
    // println!("{:#?}", interpreter.stack);
    println!("{:#?}", ast);
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    n8(i8), n16(i16), n32(i32), n64(i64), n128(i128), // Numbers (Integers)
    u8(u8), u16(u16), u32(u32), u64(u64), u128(u128), // Unsigned Numbers
    f32(f32), f64(f64),                               // Floats
    str(String),                                      // Constant String
    bool(bool),                                       // Boolean
    string(OrchidString),                             // Dynamic String
    array(OrchidArray),                               // Array (Static)
    vector(OrchidVector),                             // Vector (Dynamic)
    pointer(OrchidPointer),                           // Pointer
    object(OrchidObject),                             // Object
    nativefn(OrchidNativeFunction),                   // Native Functions
    null(i8),                                         // Null
}

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

#[derive(Debug, PartialEq, Clone)]
struct Variable {
    val_type: Type,
    is_constant: bool,
}

#[derive(Debug, Clone)]
struct Stack {
    map: HashMap<String, Variable>,
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    code: String
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidString {
    value: Box<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidArray {
    value: Vec<Type>,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidVector {
    value: Vec<Type>,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidPointer {
    pointer: String,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidObject {
    value: HashMap<String, Type>,
    keys: OrchidVector,
    values: OrchidVector,
    length: usize
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidNativeFunction {
    function: fn(Vec<Type>) -> Result<Type, String>,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidStruct {
    schema: HashMap<String, TokenType>,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidEnum {
    schema: HashMap<String, TokenType>,
    params: HashMap<String, Option<TokenType>>
}

impl OrchidString {
    fn new(value: String) -> Self {
        return OrchidString {
            value: Box::new(value)
        };
    }
    
    fn len(&self) -> usize {
        return self.value.len();
    }
    
    fn set(&mut self, new_val: String) {
        self.value = Box::new(new_val);
    }
    
    fn to_rust_string(self) -> String {
        return *self.value;
    }
    
    fn as_str(&self) -> &str {
        return &self.value;
    }
}

impl OrchidArray {
    fn new(array: Vec<Type>) -> Self {
        return OrchidArray {
            length: array.len(),
            value: array,
        };
    }
    
    fn len(&self) -> usize {
        return self.length;
    }
    
    fn get(&self) -> Vec<Type> {
        return self.value.clone();
    }
    
    fn includes(&self, search_item: &Type) -> bool {
        return self.value.clone().iter().any(|item| item == search_item);
    }
    
    fn at(&self, search_item: &Type) -> Option<usize> {
        return self.value.iter().position(|item| item == search_item);
    }
    
    fn get_index(&self, index: usize) -> Type {
        if index < self.len() {
            return self.value[index].clone();
        } else {
            panic!("BestOrchid: Unsafe Code - Cannot access index {} when length of the array is {}.", index, self.len());
        }
    }
}

impl OrchidVector {
    fn new(array: Vec<Type>) -> Self {
        return OrchidVector {
            length: array.len(),
            value: array,
        };
    }
    
    fn len(&self) -> usize {
        return self.length;
    }
    
    fn get(&self) -> Vec<Type> {
        return self.value.clone();
    }
    
    fn includes(&self, search_item: &Type) -> bool {
        return self.value.clone().iter().any(|item| item == search_item);
    }
    
    fn at(&self, search_item: &Type) -> Option<usize> {
        return self.value.iter().position(|item| item == search_item);
    }
    
    fn get_index(&self, index: usize) -> Type {
        if index < self.len() {
            return self.value[index].clone();
        } else {
            panic!("BestOrchid: Unsafe Code - Cannot access index {} when length of the vector is {}.", index, self.len());
        }
    }
    
    fn push(&mut self, value: Type) {
        self.value.push(value);
    }
    
    fn pop(&mut self) {
        self.value.pop();
    }
}

impl OrchidPointer {
    fn new(pointer: String) -> Self {
        return OrchidPointer {
            pointer,
        };
    }
    
    fn get(&self, stack: Stack) -> Variable {
        if let Some(var) = stack.map.get(&self.pointer) {
            return var.clone();
        } else {
            panic!("BestOrchid: The pointer points to an unknown variable {}", self.pointer);
        }
    }
    
    fn set(&self, mut stack: Stack, value: Type) {
        stack.set(self.pointer.clone(), value);
    }
}

impl OrchidObject {
    fn new() -> Self {
        return OrchidObject {
            value: HashMap::new(),
            keys: OrchidVector::new(Vec::new()),
            values: OrchidVector::new(Vec::new()),
            length: 0
        };
    }
    
    fn set(&mut self, key: String, value: Type) {
        self.value.insert(key.clone(), value.clone());
        self.keys.push(Type::str(key));
        self.values.push(value);
        self.length = self.length + 1;
    }
    
    fn get(&self, key: String) -> Type {
        if let Some(value) = self.value.get(&key) {
            return value.clone();
        } else {
            panic!("BestOrchid: Unknown key in object - {}", key);
        }
    }
    
    fn keys(&self) -> OrchidArray {
        return OrchidArray {
            value: self.keys.get(),
            length: self.len()
        };
    }
    
    fn values(&self) -> OrchidArray {
        return OrchidArray {
            value: self.values.get(),
            length: self.len()
        };
    }
    
    fn len(&self) -> usize {
        return self.length;
    }
}

fn stdlib_println(args: Vec<Type>) -> Result<Type, String> {
    if args.len() < 1 {
        panic!("BestOrchid: Function println has to have atleast 1 argument.");
    } else {
        println!("{:#?}", args[0]);
        
        return Ok(Type::bool(true));
    }
}

impl Stack {
    fn new() -> Self {
        return Stack {
            map: HashMap::new()
        };
    }
    
    fn push(&mut self, varname: String, value: Variable) {
        self.map.insert(varname, value);
    }
    
    fn set(&mut self, varname: String, value: Type) {
        if let Some(existing_var) = self.map.get(&varname) {
            if existing_var.is_constant {
                panic!("BestOrchid: Cannot assign to constant variable.");
            }
    
            if std::mem::discriminant(&existing_var.val_type) == std::mem::discriminant(&value) {
                match (&existing_var.val_type, &value) {
                    (Type::str(old_str), Type::str(new_str)) => {
                        if old_str.len() != new_str.len() {
                            panic!(
                                "BestOrchid: Variable with original string's bytes are not matching with new value's bytes, i.e. length of {} is not equal to {}.",
                                old_str.len(),
                                new_str.len()
                            );
                        }
                    }
                    _ => {}
                }
                
                self.map.insert(varname, Variable {
                    val_type: value,
                    is_constant: false,
                });
            } else {
                panic!(
                    "BestOrchid: Type {:#?} is not matching with type {:#?}.",
                    existing_var.val_type, value
                );
            }
        } else {
            panic!("BestOrchid: Variable `{}` not found.", varname);
        }
    }
    
    fn delete(&mut self, varname: String) {
        let val = self.get(varname.clone()).clone().val_type;
        
        match val {
            Type::nativefn(_) => {
                panic!("BestOrchid: Cannot delete a native function.");
            }
            
            _ => {
                self.map.remove(&varname);
            }
        }
    }
    
    fn get(&self, varname: String) -> &Variable {        
        if let Some(value) = self.map.get(&varname) {
            return value;
        } else {
            panic!("BestOrchid: Unknown variable or function {}.", varname);
        }
    }
    
    fn call(&self, function: String, args: Vec<Type>) {
        let val = self.get(function).clone().val_type;
        
        match val {
            Type::nativefn(func) => {
                (func.function)(args);
            }
            
            _ => {
                panic!("BestOrchid: Cannot call a non-function having type {:#?}.", val);
            }
        }
    }
    
    fn print(self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug, PartialEq, Clone)]
enum NodeType {
    Program,
    VarStmt,
    
    AssignmentExpr,
    MemberExpr,
    CallExpr,
    BinaryExpr,
    BorrowExpr,
    
    Number,
    String,
    Object,
    Vector,
    Property,
    Identifier,
}

#[derive(Debug, Clone)]
enum Expr {
    Program(Program),
    VarStmt(VarStmt),
    
    AssignmentExpr(AssignmentExpr),
    MemberExpr(MemberExpr),
    CallExpr(CallExpr),
    BinaryExpr(BinaryExpr),
    BorrowExpr(BorrowExpr),
    
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    ObjectLiteral(ObjectLiteral),
    VectorLiteral(VectorLiteral),
    Property(Property),
    Identifier(Identifier),
}

#[derive(Debug, Clone)]
struct Program {
    kind: NodeType,
    body: Vec<Expr>
}

#[derive(Debug, Clone)]
struct VarStmt {
    kind: NodeType,
    constant: bool,
    identifier: String,
    value: Vec<Expr>,
    type_annoted: TokenType,
    inferred: Option<String>,
}

#[derive(Debug, Clone)]
struct AssignmentExpr {
    kind: NodeType,
    assigne: Vec<Expr>,
    value: Vec<Expr>,
}

#[derive(Debug, Clone)]
struct MemberExpr {
    kind: NodeType,
    object: Vec<Expr>,
    property: Vec<Expr>,
    computed: bool,
}

#[derive(Debug, Clone)]
struct CallExpr {
    kind: NodeType,
    args: Vec<Expr>, // Index zero should not be accessed here.
    caller: Vec<Expr>,
}

#[derive(Debug, Clone)]
struct BinaryExpr {
    kind: NodeType,
    left: Vec<Expr>,
    right: Vec<Expr>,
    operator: String,
}

#[derive(Debug, Clone)]
struct BorrowExpr {
    kind: NodeType,
    pointer: Token,
}

#[derive(Debug, Clone)]
struct NumberLiteral {
    kind: NodeType,
    value: i128
}

#[derive(Debug, Clone)]
struct StringLiteral {
    kind: NodeType,
    value: String
}

#[derive(Debug, Clone)]
struct ObjectLiteral {
    kind: NodeType,
    properties: Vec<Property>,
}

#[derive(Debug, Clone)]
struct VectorLiteral {
    kind: NodeType,
    array: Vec<Expr>
}

#[derive(Debug, Clone)]
struct Property {
    kind: NodeType,
    key: String,
    value: Vec<Option<Expr>>,
}

#[derive(Debug, Clone)]
struct Identifier {
    kind: NodeType,
    value: String
}

impl Parser {
    fn new(code: String) -> Self {
        return Parser {
            tokens: Vec::new(),
            code,
        };
    }
    
    fn generate_ast(&mut self) -> Program {
        self.tokens = tokenize(self.code.clone());
        
        let mut program = Program {
            kind: NodeType::Program,
            body: Vec::new()
        };
        
        while self.not_eof() {
            program.body.push(self.parse_statement());
        }
        
        return program;
    }
    
    fn not_eof(&self) -> bool {
        return self.tokens[0].val_type != TokenType::EOF;
    }
    
    fn at(&self) -> Token {
        return self.tokens[0].clone();
    }
    
    fn eat(&mut self) -> Token {
        if self.tokens.is_empty() {
            panic!("BestOrchid: Parser Error - No more tokens to eat. Please reach out to BestMat.");
        } else {
            return self.tokens.remove(0);
        }
    }
    
    fn expect(&mut self, val_type: TokenType, err: &str) -> Token {
        let prev = self.eat();
        
        if prev.val_type != val_type {
            panic!("BestOrchid: Parser Error - {} - {:#?}.\nExpecting {:#?}.", err, prev, val_type);
        }
        
        return prev;
    }
    
    fn expect_type(&mut self, err: &str) -> Token {
        let prev = self.eat();
        
        if prev.value == "str" {
            return Token {
                value: prev.value,
                val_type: TokenType::STR
            };
        } else if prev.value == "string" {
            return Token {
                value: prev.value,
                val_type: TokenType::STRING
            };
        } else if prev.value == "array" {
            return Token {
                value: prev.value,
                val_type: TokenType::ARRAY
            };
        } else if prev.value == "vector" {
            return Token {
                value: prev.value,
                val_type: TokenType::VECTOR
            };
        } else if prev.value == "object" {
            return Token {
                value: prev.value,
                val_type: TokenType::OBJECT
            };
        } else if prev.value == "ptr" {
            return Token {
                value: prev.value,
                val_type: TokenType::POINTER
            };
        } else if prev.value == "n8" {
            return Token {
                value: prev.value,
                val_type: TokenType::N8
            };
        } else if prev.value == "n16" {
            return Token {
                value: prev.value,
                val_type: TokenType::N16
            };
        } else if prev.value == "n32" {
            return Token {
                value: prev.value,
                val_type: TokenType::N32
            };
        } else if prev.value == "n64" {
            return Token {
                value: prev.value,
                val_type: TokenType::N64
            };
        } else if prev.value == "n128" {
            return Token {
                value: prev.value,
                val_type: TokenType::N128
            };
        } else if prev.value == "u8" {
            return Token {
                value: prev.value,
                val_type: TokenType::U8
            };
        } else if prev.value == "u16" {
            return Token {
                value: prev.value,
                val_type: TokenType::U16
            };
        } else if prev.value == "u32" {
            return Token {
                value: prev.value,
                val_type: TokenType::U32
            };
        } else if prev.value == "u64" {
            return Token {
                value: prev.value,
                val_type: TokenType::U64
            };
        } else if prev.value == "u32" {
            return Token {
                value: prev.value,
                val_type: TokenType::U128
            };
        }
        
        panic!("BestOrchid: Parser Error - Expected variable type: found {}.", prev.value);
    }
    
    fn parse_statement(&mut self) -> Expr {
        match self.at().val_type {
            TokenType::Const | TokenType::Let => {
                let constant = self.eat().val_type == TokenType::Const;
                
                let identifier = self.expect(
                    TokenType::Identifier,
                    "Expected identifier name following let or const keywords."
                ).value;
                
                let type_annoted = self.expect_type("Expected variable type.").val_type;
                
                self.expect(TokenType::Equals, "Expected equals token identifier.");
                
                let value = self.parse_expression();
                self.expect(TokenType::Semicolon, "Missing semicolon at end of statement.");
                
                let mut inferred: Option<String> = None;
                
                match value.clone() {
                    Expr::Identifier(val) => {
                        inferred = Some(val.value);
                    }
                    
                    _ => {}
                }
                
                return Expr::VarStmt(VarStmt {
                    kind: NodeType::VarStmt,
                    identifier,
                    value: vec![value],
                    constant,
                    type_annoted,
                    inferred,
                });
            }
            
            _ => {
                return self.parse_expression();
            }
        }
    }
    
    fn parse_expression(&mut self) -> Expr {
        return self.parse_assignment_expression();
    }
    
    fn parse_assignment_expression(&mut self) -> Expr {
        let left = self.parse_object_expression();
    
        if self.at().val_type == TokenType::Equals {
          self.eat();
    
          let value = self.parse_assignment_expression();
    
          return Expr::AssignmentExpr(AssignmentExpr {
            value: vec![value],
            assigne: vec![left],
            kind: NodeType::AssignmentExpr,
          });
        }
    
        return left;
    }
     
    fn parse_vector_expression(&mut self) -> Expr {
        self.eat();
        
        let mut array: Vec<Expr> = Vec::new();
        
        while self.at().val_type != TokenType::CloseBracket {
            if self.at().val_type != TokenType::Comma {
                array.push(self.parse_expression());
            } else {
                self.eat();
            }
        }
        
        self.eat();
        
        return Expr::VectorLiteral(VectorLiteral {
            kind: NodeType::Vector,
            array,
        });
    }
    
    fn parse_object_expression(&mut self) -> Expr {
        // { Prop[] }
        if self.at().val_type != TokenType::OpenBrace {
            return self.parse_additive_expression();
        }
        
        self.eat(); // Advances Past Open Brace
        
        let mut properties: Vec<Property> = Vec::new();
        
        while self.not_eof() && self.at().val_type != TokenType::CloseBrace {
            // { key: value, key2: value2 }
            // { key }
        
            let key = self.expect(
                TokenType::Identifier,
                "Object literal key expected.",
            ).value;
        
            // Allows shorthand key: pair -> key:
            if self.at().val_type == TokenType::Comma {
                self.eat(); // Advance Past Comma
    
                properties.push(Property {
                    kind: NodeType::Property,
                    key,
                    value: Vec::new(),
                });
    
                continue;
            } else if self.at().val_type == TokenType::CloseBrace {
                properties.push(Property {
                    kind: NodeType::Property,
                    key,
                    value: Vec::new(),
                });
    
                continue;
            }
        
            self.expect(
                TokenType::Colon,
                "Missing colon following identifier in ObjectExpr",
            );
        
            let value = self.parse_expression();
        
            properties.push(Property {
                kind: NodeType::Property,
                value: vec![Some(value)],
                key,
            });
        
            if self.at().val_type != TokenType::CloseBrace {
                self.expect(
                  TokenType::Comma,
                  "Expected comma or closing bracket following Property.",
                );
              }
            }
        
            self.expect(TokenType::CloseBrace, "Object literal missing closing brace.");
        
            return Expr::ObjectLiteral(ObjectLiteral {
              kind: NodeType::Object,
              properties,
            });
    }
    
    fn parse_additive_expression(&mut self) -> Expr {
        let mut left = self.parse_multiplicitave_expression();
    
        while self.at().value == "+" || self.at().value == "-" {
          let operator = self.eat().value;
          let right = self.parse_multiplicitave_expression();
          
          left = Expr::BinaryExpr(BinaryExpr {
            kind: NodeType::BinaryExpr,
            left: vec![left],
            right: vec![right],
            operator,
          });
        }
    
        return left;
    }
    
    fn parse_multiplicitave_expression(&mut self) -> Expr {
        let mut left = self.parse_call_member_expression();
    
        while self.at().value == "/" ||
              self.at().value == "*" ||
              self.at().value == "%"
        {
          let operator = self.eat().value;
          let right = self.parse_call_member_expression();
          
          left = Expr::BinaryExpr(BinaryExpr {
            kind: NodeType::BinaryExpr,
            left: vec![left],
            right: vec![right],
            operator,
          });
        }
    
        return left;
    }
    
    fn parse_call_member_expression(&mut self) -> Expr {
        let member = self.parse_member_expr();
        
        if self.at().val_type == TokenType::OpenParam {
          return self.parse_call_expr(member);
        }
    
        return member;
    }
    
    fn parse_call_expr(&mut self, caller: Expr) -> Expr {
        let mut call_expr = Expr::CallExpr(CallExpr {
          kind: NodeType::CallExpr,
          caller: vec![caller],
          args: self.parse_args(),
        });
    
        if (self.at().val_type == TokenType::OpenParam) {
          call_expr = self.parse_call_expr(call_expr);
        }
    
        return call_expr;
    }
    
    fn parse_args(&mut self) -> Vec<Expr> {
        self.expect(TokenType::OpenParam, "Expected open parenthesis.");
        
        let mut args;
        
        if self.at().val_type == TokenType::CloseParam {
            args = Vec::new();
        } else {
            args = self.parse_arguments_list();
        }
            
        self.expect(
          TokenType::CloseParam,
          "Missing closing parenthesis inside arguments list.",
        );
        
        return args;
    }
    
    fn parse_arguments_list(&mut self) -> Vec<Expr> {
        let mut args = vec![self.parse_assignment_expression()];
    
        while self.at().val_type == TokenType::Comma {
            self.eat();
            args.push(self.parse_assignment_expression());
        }
        
        return args;
    }
    
    fn parse_member_expr(&mut self) -> Expr {
        let mut object = self.parse_primary_expression();
        
        if self.at().value == '"'.to_string() {
          self.eat();
        }
        
        while self.at().val_type == TokenType::Dot || self.at().val_type == TokenType::OpenBracket {
          let operator = self.eat();
          let property: Expr;
    
          let computed: bool;
    
          if operator.val_type == TokenType::Dot {
            computed = false;
            property = self.parse_primary_expression();
            
            match property {
                Expr::Identifier(_) => {}
                
                _ => {
                    panic!("Cannot use dot operator without right hand side being an identifier.");
                }
            }
          } else {
            computed = true;
            property = self.parse_expression();
    
            self.expect(
              TokenType::CloseBracket,
              "Missing closing bracket in computed value.",
            );
          }
          
          object = Expr::MemberExpr(MemberExpr {
                kind: NodeType::MemberExpr,
                object: vec![object],
                property: vec![property],
                computed,
            });
        }
        
        return object;
    }
    
    fn parse_primary_expression(&mut self) -> Expr {
        let token = self.at().val_type;
        
        match token {
            TokenType::Number => {
                return Expr::NumberLiteral(NumberLiteral {
                    kind: NodeType::Number,
                    value: self.eat()
                                    .value.parse::<i128>()
                                    .map_err(|e| e.to_string())
                                    .unwrap()
                });
            }
            
            TokenType::Identifier => {
                return Expr::Identifier(Identifier {
                    kind: NodeType::Identifier,
                    value: self.eat().value
                });
            }
            
            TokenType::Quote => {
                self.eat();
                let mut str = self.at().value;
                
                while self.eat().val_type != TokenType::Quote {
                    str.push_str(&self.at().value.replace("\"", ""));
                }
                
                return Expr::StringLiteral(StringLiteral {
                    kind: NodeType::String,
                    value: str
                });
            }
            
            TokenType::OpenBracket => {
                return self.parse_vector_expression();
            }
            
            TokenType::Ampersand => {
                self.eat();
                
                return Expr::BorrowExpr(BorrowExpr {
                    kind: NodeType::BorrowExpr,
                    pointer: self.eat(),
                });
            }
            
            _ => {
                panic!("BestOrchid: Invalid Token: {:#?}", self.eat());
            }
        }
    }
}



#[derive(Debug)]
struct Interpreter {
    ast: Expr,
    stack: Stack
}

impl Interpreter {
    fn new(ast: Expr) -> Self {
        return Interpreter {
            ast,
            stack: Stack::new()
        };
    }
    
    fn evaluate(&mut self, ast: Expr, mut stack: Stack) -> Type {        
        match ast {
            Expr::Program(program) => {
                let mut last_evaluated: Type = Type::null(-1);
            
                for statement in program.body {
                    last_evaluated = self.evaluate(statement, stack.clone());
                }
            
                return last_evaluated;
            }
            
            Expr::NumberLiteral(val) => {
                return Type::n128(val.value);
            }
            
            Expr::StringLiteral(val) => {
                return Type::str(val.value);
            }
            
            Expr::Identifier(val) => {
                return stack.get(val.value).clone().val_type;
            }
            
            Expr::ObjectLiteral(val) => {
                let mut obj = OrchidObject::new();
                
                let key = val.properties[0].clone().key;
                let value = val.properties[0].clone().value;
                
                let runtime_value;
                
                match value[0].clone() {
                    Some(key) => {
                        runtime_value = self.evaluate(key, stack.clone());
                    }
                    
                    None => {
                        match value[0].clone().unwrap() {
                            Expr::Identifier(ident) => {
                                runtime_value =  stack.get(ident.value).val_type.clone();
                            }
                            
                            _ => {
                                panic!("BestOrchid: Cannot interpret a variable not an identifier. This error has occured even after parsing. Please report this to BestMat Team.");
                            }
                        }
                    }
                }

                obj.set(key, runtime_value);
                
                return Type::object(obj);
            }
            
            Expr::MemberExpr(val) => {
                let varname = val.object[0].clone();
                let mut real_varname = String::new();
                
                match varname {
                    Expr::Identifier(value) => {
                        real_varname = value.value;
                    }
                    
                    _ => {}
                }
                
                let key = val.property[0].clone();
                let mut real_key = String::new();
                
                match key {
                    Expr::Identifier(value) => {
                        real_key = value.value;
                    }
                    
                    _ => {}
                }
                
                let var = stack.get(real_varname).val_type.clone();
                
                match var {
                    Type::object(obj) => {
                        return obj.get(real_key);
                    }
                    
                    _ => {
                        unimplemented!();
                    }
                }
            }
            
            Expr::BinaryExpr(val) => {
                let lhs = self.evaluate(val.left[0].clone(), stack.clone());
                let rhs = self.evaluate(val.right[0].clone(), stack.clone());
                
                let mut is_lhs_num = false;
                let mut is_rhs_num = false;
                
                let mut real_lhs = 0;
                let mut real_rhs = 0;
                
                match lhs {
                    Type::n128(num) => {
                        is_lhs_num = true;
                        real_lhs = num;
                    }
                    
                    _ => {}
                }
                
                match rhs {
                    Type::n128(num) => {
                        is_rhs_num = true;
                        real_rhs = num;
                    }
                    
                    _ => {}
                }
                
                if is_lhs_num && is_rhs_num {
                    let mut result = 0;
                    
                    match val.operator.as_str() {
                        "+" => {
                            result = real_lhs + real_rhs;
                        }
                        
                        "-" => {
                            result = real_lhs - real_rhs;
                        }
                        
                        "*" => {
                            result = real_lhs * real_rhs;
                        }
                        
                        "/" => {
                            if real_rhs == 0 {
                                panic!("BestOrchid: Cannot divide a number by zero.");
                            }
                            
                            result = real_lhs / real_rhs;
                        }
                        
                        "%" => {
                            result = real_lhs % real_rhs;
                        }
                        
                        _ => {}
                    }
                    
                    return Type::n128(result);
                }
                
                todo!();
            }
            
            Expr::AssignmentExpr(val) => {
                let mut varname = String::new();
                let mut value = val.value[0].clone();
                
                let mut real_value: Option<Type> = None;
                
                if val.kind == NodeType::Identifier {
                    match val.assigne[0].clone() {
                        Expr::Identifier(value) => {
                            varname = value.value;
                        }
                        
                        _ => {}
                    }
                    
                    let original_type = stack.get(varname.clone()).val_type.clone();
                    
                    match original_type {
                        Type::n8(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::n8((number.value) as i8)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type n8.");
                                }
                            }
                        }
                        
                        Type::n16(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::n16((number.value) as i16)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type n16.");
                                }
                            }
                        }
                        
                        Type::n32(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::n32((number.value) as i32)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type n32.");
                                }
                            }
                        }
                        
                        Type::n64(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::n64((number.value) as i64)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type n64.");
                                }
                            }
                        }
                        
                        Type::n128(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::n128(number.value)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type n128.");
                                }
                            }
                        }
                        
                        Type::u8(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::u8((number.value) as u8)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type u8.");
                                }
                            }
                        }
                        
                        Type::u16(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::u16((number.value) as u16)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type u16.");
                                }
                            }
                        }
                        
                        Type::u32(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::u32((number.value) as u32)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type u32.");
                                }
                            }
                        }
                        
                        Type::u64(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::u64((number.value) as u64)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type u64.");
                                }
                            }
                        }
                        
                        Type::u128(_) => {
                            match value {
                                Expr::NumberLiteral(number) => {
                                    real_value = Some(
                                        Type::u128((number.value) as u128)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type u128.");
                                }
                            }
                        }
                        
                        Type::str(_) => {
                            match value {
                                Expr::StringLiteral(string) => {
                                    real_value = Some(
                                        Type::str(string.value)
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type str.");
                                }
                            }
                        }
                        
                        Type::string(_) => {
                            match value {
                                Expr::StringLiteral(string) => {
                                    real_value = Some(
                                        Type::string(OrchidString::new(string.value))
                                    );
                                }
                                
                                _ => {
                                    panic!("BestOrchid: Cannot assign another type to type string.");
                                }
                            }
                        }
                        
                        _ => {
                            todo!();
                        }
                    }
                } else {
                    panic!("BestOrchid: The variable name should be an identifier.")
                }
            
                stack.set(varname, real_value.unwrap());
                
                self.stack = stack.clone();
                return Type::null(-1);
            }
            
            Expr::VarStmt(val) => {
                match val.inferred {
                    Some(var) => {
                        
                    }
                    
                    None => {
                        match val.type_annoted {
                            TokenType::N8 => {  
                                match val.value[0].clone() {
                                    Expr::NumberLiteral(number) => {
                                        self.stack.push(val.identifier, Variable { val_type: Type::n8(number.value as i8), is_constant: val.constant });    
                                    }
                                    _ => {}
                                }
                            }
                            
                            _ => {}
                        }
                    }
                }
                
                return Type::null(-1);
            }
            
            _ => {
                panic!("BestOrchid: The {:#?} AST Node hasn't been setup for implementation.", ast);
            }
        }
    }
}

// ---------------------------------------------------------------
// |                                                             |
// |******************* BESTORCHID - BESTMAT ********************|
// |                       Main Rust File                        |
// |                                                             |
// ---------------------------------------------------------------
// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.