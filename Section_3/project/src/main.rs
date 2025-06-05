/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.
 
Cast the i32 to an i16 integer and assign the result
to a separate variable.
 
Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.
 
Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.
 
Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.
 
Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.
 
Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.
 
Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

#![allow(unused)]
fn main() {
    let a: i32 = 1_337; // integer
    let b: i16 = a as i16; // casting i32 to i16
    let c: f32 = 1.55325; // float
    println!("{:.3}", c);
    
    let with_milk: bool = true; // boolean
    let with_sugar: bool = false; // boolean
    let is_my_type_of_coffee: bool = with_milk && with_sugar; // boolean expression
    let is_acceptable_coffee: bool = with_milk || with_sugar; // boolean expression
    println!("Is my type of coffee: {}", is_my_type_of_coffee); // print boolean expression
    println!("Is acceptable coffee: {}", is_acceptable_coffee); // print boolean expression

    let array: [i8; 4] = [1, 2, 3, 4]; // array
    println!("Array: {:?}", array); // print array // Debug format

    let tuple: (i32, f32, bool, [i8; 4]) = (a, c, is_my_type_of_coffee, array);
    println!("Tuple: {:#?}", tuple); // print tuple // Debug format
}