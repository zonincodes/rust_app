fn main() {
    println!("Hello, world!");

    // let indroduces a variable binding:
    let x; // declare "x"
    x = 43; // assign 42 to "x"

    // The above can be written in a single line
    let x = 42;

    // Specifying the variable type explicitly with :, that's a type annotation
    let x: i32; // `i32` is a signed 32-bit integer
    x = 42;

    // there is i8, i16, i32, i64, i128
    // also     u8, u16, u32, u64, u128 for unsigned integer

    // Can be written in a single line
    let x: i32 = 42;

    // always initialize the value before usage

    let x;
    // foobar(x); // error: borrow of possibl-unintialized variable: `x`
    x = 42;
}
