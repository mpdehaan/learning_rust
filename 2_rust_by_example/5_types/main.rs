fn main() {

    // Type annotated variable
    let a_float: f64 = 1.0;

    // This variable is an `int`
    let mut an_integer = 5i;

    // Error! The type of a variable can't be changed
    //an_integer = true;

    // This is a summary of the primitive types in Rust:
    //
    // signed integers: i8, i16, i32, i64 and int (machine word size)
    // unsigned integers: u8, u16, u32, u64 and uint (machine word size)
    // floating point: f32, f64
    // char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    // bool either true or false
    // and the unit type (), whose only value is also ()

    // ===========================
    // casting is available but must be explicit

    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);


}
