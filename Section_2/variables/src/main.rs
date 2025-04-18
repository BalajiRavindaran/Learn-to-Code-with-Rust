// Constants with type annotations
const PI: f32 = 3.14159;
const E: f32 = 2.71828;

// Type aliases
// Type aliases are used to create a new name for an existing type
type Kilometers = f32;

fn main() {
    // Print the constants
    println!("The value of PI is: {PI}");
    println!("The value of E is: {E}");

    // Print the type alias
    let distance: Kilometers = 10.0;
    println!("The distance is: {distance} kilometers");

    // Variables
    // Immutable Variables
    let steps = 5000;
    let kms = 0.8;

    // Positional arguments
    println!("I am going to walk {} steps.", steps);
    println!("I am going to run {} kilometers.", kms);
    println!("I am going to walk {} steps and run {} kilometers.", steps, kms);
    println!("I am going to walk {0} steps and run {1} kilometers, while I carry {0} Liters of water.", steps, kms);
    println!("I am going to walk {steps} steps and  run{kms} kilometers");

    // Mutable variables
    let mut distance = 0.0;
    println!("Yesterday, I walked {distance} kilometers.");
    distance += kms;
    println!("Today, I walked: {distance} kilometers.");

    // Variable shadowing
    let _miles = "miles";
    let _miles = 0.621371;
    let miles = 6;

    // Scoped variables
    // Block
    {
        let miles = 0.621371;
        println!("I walked {miles} kilometers yesterday.");
    }

    // Outside the block
    println!("I walked {miles} kilometers today.");

    // Complier Directives
    // The #[allow(unused_variables)] directive is used to suppress warnings for unused variables
    // Can be placed above functions, modules, or structs
    // Also can be placed on top of the file with `#![allow(unused_variables)]`
    #[allow(unused_variables)]
    let water = "H2O";
}
