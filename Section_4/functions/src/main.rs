fn main() {
    open_store("Downtown");
    bake_pizza(12, "pepperoni");
    println!("The square of 5 is: {}", explicit_square(5));
    println!("The implicit square of 6 is: {}", implicit_square(6));
    let mystery = mystery_function();
    println!("Mystery function returned: {:?}", mystery);

    // Demonstrating a block expression
    let x = 10;
    let calculation = {
        let y = 20;
        y + x
    };

    println!("The result of the calculation is: {}", calculation);
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store at...{}", neighborhood);
}

fn bake_pizza(number_of_toppings: i32, topping: &str) {
    println!("Baking a delicious pizza with {} {}...", number_of_toppings, topping);
}

// allow dead code to prevent warnings for unused functions
#[allow(dead_code)]
fn swim_in_profit() {
    println!("Swimming in profit!");
}

// Explicit return type for clarity
fn explicit_square(value: i32) -> i32 {
    return value * value;
}

// Implicit return type, inferred by the compiler
fn implicit_square(value: i32) -> i32 {
    value * value // No need for 'return' keyword, the last expression is returned
}

fn mystery_function() {
    // This function is intentionally left empty
    // It serves as a placeholder to demonstrate the concept of an empty function.
}