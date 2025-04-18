// Coding Assignment
/*
Create a new `about_me` project with the `cargo new` command.

Using the `println!` macro, output 3 sentences about yourself.
Feel free to invoke the macro multiple times.

From the Terminal, compile the `main.rs` file inside the `src`
folder with the Rust compiler, then manually run the executable.

From the Terminal, compile the project with the Cargo tool, then
manually run the executable.

From the Terminal, compile and run the project with a single
Cargo command.

Check your program for errors with `cargo check`.

Add a comment at the top of your source code explaining how to
compile the program for new Rust developers.

Add some spaces and line breaks to the code so that it is formatted
in an ugly manner. From the Terminal, style the code with the
`cargo fmt` command.

Replace the `println!` macro with `print!`. What happens?
*/

//////////////////////////////////////////////////////////////////////////////////////////
// To compile this program, use the command `cargo build`
// To run the program, use the command `cargo run`
// To check for errors, use the command `cargo check`
// To format the code, use the command `cargo fmt`
// To compile the main.rs file manually, use the command `rustc main.rs`
// To run the executable manually, use the command `./main` on Unix-like systems
// or `main.exe` on Windows.
// To compile and run the project with a single Cargo command, use `cargo run`
// To create a new project, use the command `cargo new project_name`
// To create a new binary project, use the command `cargo new --bin project_name`
// To create a new library project, use the command `cargo new --lib project_name`
// To create a new project with a specific name, use the command `cargo new project_name`

fn main() {
    println!("Hello, my name is John Doe.");
    println!("I am a software developer.");
    println!("I love learning new programming languages.");
    println!("I enjoy solving complex problems.");
    print!("I am excited to learn Rust!   "); // This will not add a new line
    println!("I am looking forward to building amazing projects.");
}
