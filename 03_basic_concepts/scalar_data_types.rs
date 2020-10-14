fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    // Available integer types
    // Length - Signed - Unsigned
    // 8 - i8 - u8
    // 16 - i16 - u16
    // 32 - i32 - u32 (DEFAULT)
    // 64 - i64 - u64
    // 128 - i128 - u128
    // arch - isize - usize (COMPUTER ARCHITECTURE DEPENDENT)
    let decimal_int = 98_222;
    let hex_int = 0xff;
    let octal_int = 0o77;
    let binary_int = 0b1111_1001;
    let byte_u8 = b'A'; // u8 Only

    // Available floating point types
    // f32 (32bits)
    // f64 (64bits) (DEFAULT)
    let float_64 = 2.0;
    let float_32: f32 = 3.0;

    // Boolean: true/false
    let t = true;
    let f = false;

    // Char types
    // 4 bytes in size
    // Unicode
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}