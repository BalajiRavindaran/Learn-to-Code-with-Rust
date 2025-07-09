/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.
 
Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.
 
Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.
 
The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.
 
In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.
 
Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/

fn main() {
    let is_concert: bool = true;
    let is_event: bool = is_concert;
    println!("The ownership is not moved, they are copied for bool");
    println!("is_concert: {}, is_event: {}\n", is_concert, is_event);

    let sushi: &str = "Salmon";
    let dinner: &str = sushi;
    println!("The ownership is not moved for string literals as they are hard coded into the binary, the reference is copied");
    println!("sushi: {}, dinner: {}\n", sushi, dinner);

    let sushi_on_heap: String = String::from("Tuna");
    let dinner_on_heap: String = sushi_on_heap; // Cloning creates a deep
    println!("The ownership is moved from the sushi_on_heap to dinner_on_heap as String is a heap allocated type");
    // println!("sushi_on_heap: {}, dinner_on_heap: {}", sushi_on_heap, dinner_on_heap); // This line would cause a compile error because sushi_on_heap is no longer valid
    println!("dinner_on_heap: {}\n", dinner_on_heap);

    let dinner_after_eat_with_clone = eat_meal(dinner_on_heap.clone() + " - Clone string");
    println!("dinner_on_heap after eat_meal(Passing in a clone string)(Cleared the cloned data on the heap): {}", dinner_after_eat_with_clone);
    println!("dinner_on_heap is still valid after cloning, dinner_on_heap: {}\n", dinner_on_heap);

    println!("The ownership of meal is moved from the main function to the eat_meal function");
    println!("meal is also given the mut attribute in the called function, so it can be modified");
    println!("In the called function, the data in the heap is cleared using the clear() method");
    println!("Finally the meal ownership is returned to the main function and stored in the dinner_after_eat variable");
    let dinner_after_eat = eat_meal(dinner_on_heap);
    println!("dinner_on_heap after eat_meal(Cleared the data on the heap): {}\n", dinner_after_eat);
}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}