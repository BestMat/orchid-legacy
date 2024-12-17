// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.

use lexer::Lexer;

#[path="src/lexer/lexer.rs"] mod lexer;

fn main() {
    println!("I Love You Amma");
    
    let mut lexer = Lexer::new("(hello!123@ ");
    lexer.print_tokens();
}