// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// * Use a function to display your first name
fn first_name(f: &str) -> &str {
    return f;
}

// * Use a function to display your last name
fn last_name(l: &str) -> &str {
    return l;
}

// * Use the println macro to display messages to the terminal
fn main() {
    let first_n = first_name("Ivan");
    let last_n = last_name("Kovnatsky");

    println!("My name is \"{} {}\"", first_n, last_n);
}
