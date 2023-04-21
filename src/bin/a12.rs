// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
enum Color {
    Red,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("color: red"),
            Color::Blue => println!("color: blue"),
        }
    }
}

// * Use a struct to encapsulate the box characteristics
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl Box {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let r = Box::new(
        6.22,
        Color::Red,
        Dimensions {
            width: 9.22,
            height: 32.89,
            depth: 28.67,
        },
    );

    let b = Box::new(
        6.22,
        Color::Blue,
        Dimensions {
            width: 9.22,
            height: 32.89,
            depth: 28.67,
        },
    );

    r.print();
    b.print();
}
