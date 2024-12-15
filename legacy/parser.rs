// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.
#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    code: String
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
    
        if self.at().val_type == TokenType::OpenParam {
          call_expr = self.parse_call_expr(call_expr);
        }
        
        self.expect(TokenType::Semicolon, "Missing semicolon.");
    
        return call_expr;
    }
    
    fn parse_args(&mut self) -> Vec<Expr> {
        self.expect(TokenType::OpenParam, "Expected open parenthesis.");
        
        let args;
        
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

// Tests:
// let code = String::from(fs::read_to_string("test/main.orc").unwrap());
// let mut parser = Parser::new(code);
// let ast = parser.generate_ast();
// println!("{:#?}", ast);