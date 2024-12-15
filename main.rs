// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.
// ---------------------------------------------------------------
// |                                                             |
// |******************* BESTORCHID - BESTMAT ********************|
// |                       Main Rust File                        |
// |                                                             |
// ---------------------------------------------------------------

fn main() {
    println!("I Love You Amma");
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    N8(i8), N16(i16), N32(i32), N64(i64), N128(i128), // Numbers (Integers)
    U8(u8), U16(u16), U32(u32), U64(u64), U128(u128), // Unsigned Numbers
    F32(f32), F64(f64),                               // Floats
    STR(String),                                      // Constant String
    BOOL(bool),                                       // Boolean
}

// ---------------------------------------------------------------
// |                                                             |
// |******************* BESTORCHID - BESTMAT ********************|
// |                       Main Rust File                        |
// |                                                             |
// ---------------------------------------------------------------
// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.