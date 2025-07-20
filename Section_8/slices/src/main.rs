fn main() {
    let name: String = String::from("Balaji Ravindaran");
    let name_slice: &str = &name[0..6]; // This is a string literal slice in bytes and not character length, this is not a reference to a reference type, but a reference to a chunk of memory
    println!("Name slice 1: {}", name_slice);
    let name_slice2: &str = &name[7..];
    println!("Name slice 2: {}", name_slice2);

    let name2: &str = {
        let temp: &str = "Balaji Ravindaran";
        &temp[0..6] // This is not a dangling reference because temp is a temporary variable that goes out of scope after this block, but the string literal itself has a static lifetime
    }; // Even after temp goes out of scope, name2 is still valid because it is a string literal, which has a static lifetime in the Binary code
    println!("Name 2: {}", name2);

    let emoji: &str = "😀";
    println!("Emoji: {}", emoji.len()); // This will return 4
    // println!("Emoji: {}", &emoji[0..1]); // This is will throgh an error - byte index 1 is not a char boundary; it is inside '😀' (bytes 0..4) of `😀`
    println!("Emoji slice: {}", &emoji[0..4]); // This is valid as it slices the emoji correctly in bytes

    do_hero_stuff(&name);
    do_hero_stuff(name2);

    let mut arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let arr_slice: &[i32] = &arr[0..3]; // Here unlike slices are not mentioned in bytes, but in terms of elements, so this is a slice of the first 3 elements of the array
    println!("Array slice: {:?}", arr_slice);
    print_length(arr_slice);

    let arr_slice2: &mut [i32] = &mut arr[3..]; // This is a mutable slice of the array
    println!("Array slice 2: {:?}", arr_slice2);
    print_length(arr_slice2);
    
    arr_slice2[0] = 100; // This is valid because arr_slice2 is a mutable slice of the array, so you can modify the elements of the array through the slice
    println!("Modified array slice 2: {:?}", arr_slice2);
    println!("Modified array: {:?}", arr); // This will show the modified array with the first element changed to 100

    let arr_slice3: &[i32; 6] = &arr; // Note the type declaration, This is a reference, which is equivalent to &arr[0..6]
    println!("Array slice 3: {:?}", arr_slice3);
    print_length(arr_slice3); // This works because Rust automatically converts the array slice to a slice of i32, so you can pass a slice of an array to a function that expects a slice of i32, without needing to explicitly convert it.

}
// Deref Coercion is a feature in Rust that allows you to automatically convert a reference of one type to a reference of another type, as long as the target type implements the Deref trait. This is particularly useful when working with types like String and &str, where you can pass a String to a function that expects a &str, without needing to explicitly convert it.
// This is because Rust automatically dereferences the String to a &str when needed, making it easier to work with different string types without having to manually convert them every time.
fn do_hero_stuff(hero_name: &str) { // This support both String and &str types, because behind the scenes, Rust converts String to &str, but &String doesnt not convert &str to &String, so reverse mapping doesnt work
    println!("Hero name: {}", hero_name);
}

// Rust Deref Coercion converts any array reference or slices to a common type of &[i32] or &[i32; N] where N is the length of the array, so you can pass a slice of an array to a function that expects a slice of i32, without needing to explicitly convert it.
// This is useful when you want to work with a slice of an array without having to worry
// about the specific type of the slice, as long as it is a slice of i32
// or an array of i32 with a known length &[i32, 6].
fn print_length(s: &[i32]) {
    println!("Length of array slice: {}", s.len());
}