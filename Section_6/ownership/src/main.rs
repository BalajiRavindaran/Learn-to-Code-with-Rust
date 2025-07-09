// Ownership in Rust
// Ownership is a key concept in Rust that ensures memory safety without needing a garbage collector.
// Each value in Rust has a single owner, and when the owner goes out of scope,
// the value is dropped (deallocated).

#![allow(unused)]
fn main() {

    // Static types like int, bool, char, floting point, etc. are stored on the stack.
    // The stack is a region of memory that stores values in a last-in, first-out
    // (LIFO) manner. When a function is called, its local variables are pushed
    // onto the stack, and when the function returns, those variables are popped off.
    
    let age: i32 = 30; // age is the owner of the value 30
    let is_alive: bool = true; // is_alive is the owner of the value true
    let age_copy: i32 = age; // age_copy uses the copy trait to create a copy of age
    let age_copy_ref: &i32 = &age; // age_copy_ref is a reference to the value owned by age
    // References also have the copy trait, which means that they can be copied
    // without taking ownership of the value. This is because references are
    // just pointers to the value, and they do not own the value itself.

    println!("Display is Age: {}", age); // the println! macro with the display trait is used to print the value of age
    println!("Debug Is alive: {:?}", is_alive); // the println! macro with the debug trait is used to print the value of is_alive

    // Two types of strings in Rust:

    // 1. String slice (&str): A reference to a string literal, which is
    //    embedded in the binary and is immutable.
    // & - says that &str is a reference type, which means it points to the data directly embedded in the binary file.
    let name_str: &str = "Alice"; // name_str is a string slice, which is a reference to a string literal, this type is embedded in the binary
    // 2. String: An owned string type that is allocated on the heap and can
    //    be modified. It is dynamically sized and can grow or shrink in size.
    let emplty_string: String = String::new(); // empty_string is a String, which is an owned string type, this type is allocated on the heap
    let name_string: String = String::from("Alice"); // name_string is a String, which is an owned string type, this type is allocated on the heap

    // Ownership movement in Rust:

    let person_name: String = String::from("Alice"); // person_name is a heap-allocated string, which doesn't have the copy trait
    // When we assign person_name to another variable, ownership is moved.
    // This means that person_name is no longer valid, and we cannot use it after this
    // assignment. The new variable becomes the owner of the value.
    // If we try to use person_name after this line, it will result in a compile error.
    // This is because Rust enforces strict ownership rules to prevent dangling pointers
    // and ensure memory safety.
    let another_person_name: String = person_name; // ownership of person_name is moved to another_person_name
    //println!("Person name: {}", person_name); // This line would cause a compile error because person_name is no longer valid.
    println!("Another person name: {}", another_person_name); // This line is valid because another_person_name is the new owner of the value.

    let another_person_name_cloned: String = another_person_name.clone(); // Cloning creates a deep copy of the value, allowing us to have two valid owners.
    println!("Another person name valid owner after cloning: {}", another_person_name); // This line is valid because another_person_name is still the owner of the value.
    println!("Another person name cloned: {}", another_person_name_cloned); // This line is valid because another_person_name_cloned is a separate owner.
    drop(another_person_name); // Explicitly dropping the value, which is optional in Rust. It will be dropped automatically when it goes out of scope.

    // Borrowing in Rust (& is the borrow operator):
    // Works for both heap and stack allocated values.

    // Stack allocated values can be borrowed using references.
    let owner_int: i32 = 42; // owner_int is an owner of the value 42
    let borrowed_int: &i32 = &owner_int; // borrowed_int is a reference
    // to the value owned by owner_int. This is called borrowing.
    println!("Borrowed integer using explicit dereferencing operator - *: {}", *borrowed_int); // We can use the borrowed value without
    // taking ownership of it. The owner_int remains valid and can still be used.

    // Heap allocated values can also be borrowed using references.
    let owner_string: String = String::from("Hello, Rust!"); // owner_string is an owner of the heap-allocated string
    let borrowed_string: &String = &owner_string; // borrowed_string is a reference to the value owned by owner_string
    println!("Borrowed string: {}", borrowed_string); // We can use the borrowed value without taking ownership of it.

    let mut mutable_string: String = String::from("Hello, Rust!"); // mutable_string is an owner of the heap-allocated string
    println!("Mutable string before modification: {}", mutable_string); // Print the original value
    let mut returned_value = print_my_value(age, name_string, mutable_string); // Passing values to a function, which takes ownership of the values.
    returned_value.push_str(" - modified in main"); // We can modify the returned value because it is mutable.
    println!("Ownership moved from the called function to the main function: {}", returned_value);
    println!("{age} is valid after function call, because it is a stack allocated value and has the copy trait."); // age is still valid because it is a stack-allocated value with the copy trait.
    println!("Name string is not valid after function call, because it is a heap allocated value and does not have the copy trait."); // name_string is no longer valid after the function call because it was moved into the function.
    println!("When a heap data ownership is moved, the mutation dynamics of the value is not moved with it, thus we need to mention mut on the new moved owner as well."); // mutable_string is no longer valid after the function call because it was moved into the function and modified.
} // drop is a function that takes ownership of the value and deallocates it, this is called automatically on all variables when the block ends.


fn print_my_value(value1: i32, value2: String, mut value3: String) -> String {
    // This function takes ownership of value1 and value2.
    // value1 is a stack-allocated integer, thus the value is copied.
    // value2 is a heap-allocated string, thus the ownership is moved.
    println!("Value 1: {}, Value 2: {}", value1, value2);
    value3.push_str(" - modified"); // We can modify value3 because it is mutable.
    println!("Value 3: {}", value3);
    
    let value4: String = String::from("New String in the Called Function"); // value4 is a new heap-allocated string.
    println!("Value 4: {}", value4); // Print the new value.
    return value4;

    
    // After this function call, value1 and value2 will be dropped automatically
    // when they go out of scope at the end of the function.
}

// age and is_alive are both owned by the main function.
// When the main function ends, both variables go out of scope and are dropped.
// This means that the memory allocated for these variables is automatically reclaimed.
// This ownership model prevents memory leaks and ensures that memory is managed safely.