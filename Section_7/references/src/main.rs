#[allow(unused_variables)]
fn main() {
    let mut meal = String::from("Salmon");
    println!("Initial meal: {}", meal);

    add_meal(&mut meal);
    print_meal(&meal);

    // Multiple immutable references to an owned values is allowed
    let meal_ref1 = &meal;
    let meal_ref2 = &meal;
    println!("Meal Reference 1 (Immutable): {}\nMeal Reference 2 (Immutable): {}", meal_ref1, meal_ref2);

    // Mutable references to an owned value are exclusive
    // The lifetime of a mutable reference must not overlap with any immutable references
    // This means you cannot have a mutable reference while immutable references exist
    let meal_ref3 = &mut meal; // Another mutable reference
    println!("Meal Reference 3 (Mutable): {}", meal_ref3);
    let meal_ref4 = &meal; // This will cause a compile error if uncommented
    println!("Meal Reference 4 (Immutable): {}", meal_ref4);
    // println!("Final meal after all operations: {}", meal_ref3); // This line would cause a compile error - error[E0502]: cannot borrow `meal` as immutable because it is also borrowed as mutable

    // Immutable reference have copy traits, so they can be copied
    let meal_copy = meal_ref4;
    println!("Meal Copy from Immutable Reference 4: {}", meal_copy);

    // Mutable references does not have copy traits, so the ownership is moved
    // let meal_copy_mut = meal_ref3; // Ownership is moved here
    // println!("Meal Copy from Mutable Reference 3: {}", meal_copy_mut);
    // println!("Meal Reference 3 (Mutable): {}", meal_ref3); // This line would cause a compile error

    let arr1: [i32; 3] = [1, 2, 3];
    let arr1_0 = arr1[0]; // Int has copy trait, so this performs a copy
    println!("Array Element 0: {}", arr1_0);

    let arr2: [String; 3] = [String::from("One"), String::from("Two"), String::from("Three")];
    // let arr2_0 = arr2[0]; // This will cause a compile error because String does not implement the Copy trait, and partial moves are not allowed
    let arr2_0 = &arr2[0]; // String does not have copy trait, so this creates a reference
    println!("Array Element 0 (Reference): {}", arr2_0);
}

// Mutable reference to a String
fn add_meal(meal: &mut String) {
    meal.push_str(" with rice");
    println!("Updated meal after Mutable Reference: {}", meal);
} 

// Immutable reference to a String
fn print_meal(meal: &String) {
    println!("Meal on Immutable Reference: {}", meal);
}

// This returns a reference to a String, but the String is dropped at the end of the function
// This will create a dangling reference, This is caught by the Rust compiler
// fn create_meal() -> &String {
//     let meal = String::from("Salmon");
//     &meal // This will cause a compile error because `meal` is dropped at the end of this function
// }