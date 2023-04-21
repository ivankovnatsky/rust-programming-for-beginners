// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct Shoe(Color);

#[derive(Debug)]
struct Shirt(Color);

#[derive(Debug)]
struct Pants(Color);

impl Shoe {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shoe(color: Shoe) {
    println!("{:?}", color);
}

fn print_shirt(color: Shirt) {
    println!("{:?}", color);
}

fn print_pants(color: Pants) {
    println!("{:?}", color);
}

fn main() {
    let h = Shoe::new(Color::Gray);
    print_shoe(h);

    let s = Shirt::new(Color::Purple);
    print_shirt(s);

    let p = Pants::new(Color::Yellow);
    print_pants(p);

    let p = Pants::new(Color::Custom("Goeryasdf".to_owned()));
    print_pants(p);
}
