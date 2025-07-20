/*
Define a `cereals` array with 5 heap Strings
  - Cookie Crisp
  - Cinnamon Toast Crunch
  - Frosted Flakes
  - Cocoa Puffs
  - Captain Crunch
 
Declare a `first_two` variable that extracts a slice
of the first two cereals. Print the slice.
 
Declare a `mid_three` variable that extracts a slice
of the middle three cereals (Cinnamon Toast Crunch
up to and including Cocoa Puffs). Print the slice.
 
Declare a `last_three` variable that extracts a slice
of the last three cereals. Print the slice.
 
Using the `last_three` slice, target the last element
("Captain Crunch") and replace it with "Lucky Charms".
Print the complete `cereals` array.
 
Declare a `cookie_crisp` variable that references the
"Cookie Crisp" String.
 
Declare a `cookie` variable that extracts a slice of
the text "Cookie" from the String. Print the slice.
 
Declare a `cocoa_puffs` variable. Make it a reference
to the "Cocoa Puffs" String (in other words, a &String).
 
Declare a `puffs` variable that extracts a slice of
the text "Puffs" from the String. Print the slice.
*/

fn main() {
    let mut cereals: [String; 5] = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch")
    ];

    let first_two: &[String] = &cereals[..2];
    println!("First two cereals: {:?}", first_two);

    let middle_three: &[String] = &cereals[2..5];
    println!("Middle three cereals: {:?}", middle_three);

    let last_three: &mut [String] = &mut cereals[2..];
    println!("Last three cereals: {:?}", last_three);

    last_three[last_three.len() - 1] = String::from("Captain Crunch");
    println!("Updated last three cereals: {:?}", last_three);
    println!("All cereals: {:?}", cereals);

    let cookie_crisp: &String = &cereals[0];
    println!("First cereal: {}", cookie_crisp);

    let cookie: &str = &cookie_crisp[..6];
    println!("First cereal substring: {}", cookie);

    let cocoa_puffs: &String = &cereals[3];
    println!("Fourth cereal: {}", cocoa_puffs);

    let puffs: &str = &cocoa_puffs[6..];
    println!("Fourth cereal substring: {}", puffs);
}
