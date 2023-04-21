// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    color: String,
}

// * The name and colors should be printed using a function
fn print_persons_info(name: &str, color: &str) {
    println!("name: {:?}, color: {:?}", name, color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let p = vec![
        Person {
            age: 11,
            name: "Name".to_owned(),
            color: "Blue".to_owned(),
        },
        Person {
            age: 8,
            name: "Yuo".to_owned(),
            color: "Red".to_owned(),
        },
        Person {
            age: 5,
            name: "Bao".to_owned(),
            color: "Gray".to_owned(),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for i in p {
        if i.age <= 10 {
            print_persons_info(&i.name, &i.color);
        }
    }
}
