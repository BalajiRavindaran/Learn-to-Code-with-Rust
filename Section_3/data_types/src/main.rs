#![allow(unused)]
fn main() {
    // Integers
    let a: i8 = 5; // 8-bit signed integer: -128 to 127
    let b: u8 = 5; // 8-bit unsigned integer: 0 to 255
    let e: i16 = 5; // 16-bit signed integer: -32_768 to 32_767
    let f: u16 = 5; // 16-bit unsigned integer: 0 to 65_535
    let x: i32 = 5; // 32-bit signed integer: -2_147_483_648 to 2_147_483_647
    let y: u32 = 5; // 32-bit unsigned integer: 0 to 4_294_967_295
    let z: i64 = 5; // 64-bit signed integer: -9_223_372_036_854_775_808 to 9_223_372_036_854_775_807
    let w: u64 = 5; // 64-bit unsigned integer: 0 to 18_446_744_073_709_551_615
    let g: i128 = 5; // 128-bit signed integer: -170_141_183_460_469_231_731_687_303_715_884_105_728 to 170_141_183_460_469_231_731_687_303_715_884_105_727
    let h: u128 = 5; // 128-bit unsigned integer: 0 to 340_282_366_920_938_463_463_374_607_431_768_211_455
    
    // let aa = 5i8; // type can be specified with a suffix also
    // let bb = 5u8; // type can be specified with a suffix also

    // let aa: i8 = 128; // through overflow error, range is -128 to 127
    // let bb: u8 = 256; // through overflow error, range is 0 to 255

    let number_: i32 = 3_632_125; // Numbers can be separated with underscores for readability

    // i/usize is the size of the pointer, so it is 32-bit on 32-bit systems and 64-bit on 64-bit systems
    let i: isize = 5; // signed integer
    let j: usize = 5; // unsigned integer
    
    // Strings - \n, \t, \", \', \\, r# raw string
    let s: &str = "Hello, world!\nThis is a new line\tand this is a tab, these are quotes \'\" and this is a back slash \\."; // string slice
    let t: &str = r#"This is a raw string with "quotes" and \n new line and so on."#; // raw string

    println!("String: {}", s);
    println!("Raw String: {}", t);

    // Methods
    let value: i32 = -15;
    println!("Value ABS: {}", value.abs()); // absolute value
    println!("Value POW(2): {}", value.pow(2)); // Power of 2

    // Floating Point Numbers
    let f1: f32 = 3.1415; // 32-bit floating point number: -3.40282347E+38 to 3.40282347E+38
    let f2: f64 = 3.1415; // 64-bit floating point number: -1.7976931348623157E+308 to 1.7976931348623157E+308

    println!("Float 32: {}", f1);
    println!("Float 64: {}", f2);

    // Format Specifiers
    println!("Format Specifier - Type 1: {f1:.2}"); // default
    println!("Format Specifier - Type 2: {:.2}", f2); // additional
    println!("Format Specifier - Type 3: {0:.2}", f1); // positional

    // Casting
    let a: i32 = 5;
    let b: f32 = a as f32 + 0.5; // casting i32 to f32
    let c: i32 = b as i32; // casting f32 to i32
    let d: i32 = 5.5 as i32; // casting f32 to i32, truncating the decimal part
    let e: i32 = 5.5_f32.round() as i32; // rounding the float to the nearest integer | _f32 is a type suffix
    println!("Casting: {} {} {} {}", b, c, d, e);

    // Math Operations
    let addition: i32 = 5 + 5; // addition
    let subtraction: i32 = 5 - 5; // subtraction
    let multiplication: i32 = 5 * 5; // multiplication
    println!("Addition: {}", addition);
    println!("Subtraction: {}", subtraction);
    println!("Multiplication: {}", multiplication);

    let floor_division: i32 = 5 / 2; // floor division
    let decimal_division: f32 = 5f32 / 2f32; // decimal division
    println!("Floor Division: {}", floor_division); // print floor division result
    println!("Decimal Division: {}", decimal_division); // print decimal division result
    
    let remainder: i32 = 5 % 2; // remainder
    println!("Remainder: {}", remainder); // print remainder result

    // Augmented Assignment
    let mut x: i32 = 5;
    x += 5; // x = x + 5 | +=, -=, *=, /= etc etc
    println!("Augmented Assignment: {}", x); // print augmented assignment result
    
    // Boolean
    let is_true: bool = true; // boolean value
    let is_false: bool = false; // boolean value
    println!("Boolean: {} {}", is_true, is_false); // print boolean values
    println!("Boolean: {} {}", !is_true, !is_false); // print negated boolean values
    
    let age: i32 = 18; // age
    let is_adult: bool = age >= 18; // boolean expression  
    println!("Is Adult: {}", is_adult); // print adult status

    // Equality
    let a: i32 = 5; // integer
    let b: i32 = 5; // integer
    let c: i32 = 6; // integer
    println!("Equality: {} {} {}", a == b, a != c, a > c); // print equality, inequality and greater than
    println!("Equality: {} {} {}", a < b, a <= c, a >= c); // print less than, less than or equal to and greater than or equal to
    println!("Equality: {} {} {}", a.eq(&b), a.ne(&c), a.gt(&c)); // print equality, inequality and greater than using methods
    println!("Equality: {} {} {}", a.lt(&b), a.le(&c), a.ge(&c)); // print less than, less than or equal to and greater than or equal to using methods

    // And & Or
    let a: bool = true; // boolean
    let b: bool = false; // boolean
    println!("And: {}", a && b); // print and
    println!("Or: {}", a || b); // print or
    println!("Not: {}", !a); // print not

    // Character
    let c: char = 'a'; // character
    let d: char = 'A'; // character
    let e: char = '1'; // character
    let f: char = '!'; // character
    let g: char = ' '; // character
    let h: char = '\n'; // character
    let i: char = '\t'; // character
    let j: char = '\u{1F600}'; // character
    println!("Character: {} {} {} {} {} {} {} {}", c, d, e, f, g, h, i, j); // print characters
    println!("Character: {} {} {} {} {} {} {} {}", c.is_alphabetic(), d.is_alphabetic(), e.is_numeric(), f.is_alphanumeric(), g.is_whitespace(), h.is_control(), i.is_digit(10), j.is_ascii()); // print character properties

    // Array
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // array of integers
    let arr2: [i32; 5] = [1; 5]; // array of integers with same value
    println!("Array: {:?}", arr); // print array // Debug trait
    println!("Array: {:?}", arr2); // print array
    println!("Array Length: {}", arr.len()); // print array length // Display trait
    println!("Array First Element: {}", arr[0]); // print first element of array
    println!("Array Last Element: {}", arr[arr.len() - 1]); // print last element of array
    println!("Array First Element: {}", arr.first().unwrap()); // print first element of array using method
    println!("Array Last Element: {}", arr.last().unwrap()); // print last element of array using method
    println!("Array Slice: {:?}", &arr[1..3]); // print slice of array
    println!("Array Slice: {:?}", &arr[1..]); // print slice of array
    println!("Array Slice: {:#?}", &arr[..3]); // print slice of array // pretty print

    arr[0] = 10; // change first element of array
    println!("Array: {:?}", arr); // print array

    // Debug Macro
    dbg!(2 + 2); // print debug information
    dbg!(arr); // print debug information

    // Tuple
    let tuple: (i32, f32, char) = (5, 3.14, 'a'); // tuple of integers, floats and characters
    let (a, b, c) = tuple; // destructuring tuple
    println!("Tuple: {} {} {}", a, b, c); // print tuple
    println!("Tuple: {:?}", tuple); // print tuple

    // Range
    let range: std::ops::Range<i32> = 1..arr[0]; // range of integers
    let range2: std::ops::RangeInclusive<i32> = 1..=5; // inclusive range of integers
    println!("Range: {:?}", range); // print range
    println!("Range: {:?}", range2); // print range
    for i in range {
        if i == arr[0] - 1 {
            println!("{}", i); // print range
            break; // break the loop
        }
        print!("{}", i); // print range
    }
    for i in range2 {
        print!("{}", i); // print range
    }
}
