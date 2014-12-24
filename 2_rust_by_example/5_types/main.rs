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

    // ======================================================
    // literals
    // Suffixed literals, their types are known at initialization

    let x = 1u8;
    let y = 2u;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // Constraints (summands must have the same type) for `i` and `f`
    let _constraint_i = x + i;
    let _constraint_f = z + f;
    // TODO ^ Try commenting out these two lines


}
