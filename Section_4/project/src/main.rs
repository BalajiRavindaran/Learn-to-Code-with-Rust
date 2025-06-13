fn main() {
    apply_to_jobs(12, "engineer");
    println!("{}", is_even(2));
    println!("{:?}", alphabets("hello world, and z"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs.");
}

fn is_even(number: i32) -> bool {
    number%2 == 0
}

fn alphabets(string: &str) -> (bool, bool) {
    return (string.contains("a"), string.contains("z"));
}