fn main() {
    let some_boolean: bool = true; // boolean value
    let season: &str = "winter"; // string slice

    // If statement
    if some_boolean {
        println!("The if statement says: The boolean is true!"); // print if true
    }

    // If-else if-else statement
    if season == "summer" {
        println!("The if statement says: It's summer!"); // print if summer
    } else if season == "winter" {
        println!("The else if statement says: It's winter!"); // print if winter
    } else {
        println!("The else statement says: It's not summer or winter!"); // print if neither
    }

    // If expression
    // The if expression returns a value, which can be assigned to a variable
    // This is different from the if statement, which does not return a value
    let result: i32 = if some_boolean {
        // if expression
        10 // return 10 if true
    } else {
        20 // return 20 if false
    };
    println!("The variable assignment result is: {}", result); // print result of if expression

    let evaluation: bool = true; // boolean value for match expression

    // Match expression
    // The match expression is similar to a switch statement in other languages
    // It allows you to match a value against a series of patterns
    // Each arm of the match expression can return a value, which can be assigned to a variable
    // The match expression is exhaustive, meaning it must cover all possible values of the matched type
    // In this case, we are matching a boolean value
    let value: i32 = match evaluation {
        // match expression
        true => 20, // return 20 if true
        false => 30, // return 30 if false
                     // Note: The catch-all arm is not necessary for boolean values, but it's good practice to include it for exhaustiveness
    };

    println!("Match expression result is: {}", value); // print value from match expression

    let number: i32 = 5; // integer value for loop example

    // Match expression with multiple values
    // This match expression checks if the number is even or odd
    // It uses guards to check the condition for each arm
    // The match expression is exhaustive, meaning it must cover all possible values of the matched type
    // value assignment is not necessary here, but it's good practice to include it for clarity
    match number {
        // match expression for loop
        value if value % 2 == 0 => {
            // check if number is even
            println!("Match with value result: The number {} is even.", value); // print if even
        }
        value if value % 2 != 0 => {
            // check if number is odd
            println!("Match with value result: The number {} is odd.", value); // print if odd
        }
        _ => unreachable!(), // catch-all arm, should never be reached
                             // Note: The catch-all arm is not necessary for this case, but it's good practice to include it for exhaustiveness
    }

    // Loop example
    // This loop demonstrates the use of a mutable counter variable
    // and the use of continue and break statements
    // The loop will run indefinitely until the break statement is reached
    let mut counter: i32 = 15; // mutable counter variable
    loop {
        // infinite loop
        counter -= 1; // decrement counter
        println!("Loop iteration: {}", counter); // print current iteration

        if counter == 3 {
            // check if counter reached 3
            println!("Continuing the loop at iteration: {}", counter); // print continue message
            continue; // skip to next iteration
        }

        if counter <= 0 {
            // check if counter reached 0
            println!("Breaking the loop at iteration: {}", counter); // print break message
            break; // exit the loop
        }
    }

    let mut counter: i32 = 15; // mutable counter variable
                               // While loop example
    while counter > 0 {
        // infinite loop
        counter -= 1; // decrement counter
        println!("while loop iteration: {}", counter); // print current iteration

        if counter == 3 {
            // check if counter reached 3
            println!("Continuing the while loop at iteration: {}", counter); // print continue message
            continue; // skip to next iteration
        }

        if counter <= 0 {
            // check if counter reached 0
            println!("Breaking the while loop at iteration: {}", counter); // print break message
        }
    }

    let number: u32 = 5; // number to calculate factorial
    let result: u32 = factorial(number); // call factorial function
    println!("The factorial of {} is: {}", number, result); // print result of factorial
                                                            // Note: The factorial function is a simple example of recursion
                                                            // In practice, recursion can lead to stack overflow for large inputs
                                                            // It's important to ensure that the base case is reached to avoid infinite recursion
                                                            // Tail recursion is a technique that can optimize recursive functions to avoid stack overflow
                                                            // However, Rust does not currently support tail call optimization, so care must be taken with
}

// Recursion example
// This function demonstrates the use of recursion to calculate the factorial of a number
fn factorial(n: u32) -> u32 {
    if n == 0 {
        // base case
        1 // return 1 if n is 0
    } else {
        // recursive case
        n * factorial(n - 1) // return n multiplied by factorial of n-1
    }
}
