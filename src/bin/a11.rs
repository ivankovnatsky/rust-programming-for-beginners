// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Groceries {
    quantity: i32,
    id: i32,
}

fn print_quantity(i: &Groceries) {
    println!("quantity is {:?}", i.quantity);
}

fn print_id(i: &Groceries) {
    println!("id is {:?}", i.id);
}

fn main() {
    let carrot = Groceries {
        quantity: 5,
        id: 1234,
    };

    print_quantity(&carrot);
    print_id(&carrot);
}
