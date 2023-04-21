// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

#[derive(Debug)]
enum Furniture {
    Chair,
    Bed,
    Table,
    Couch,
}

fn main() {
    let mut items_in_stock = HashMap::new();
    let mut total_number_of_items = 0;

    items_in_stock.insert(5, Furniture::Chair);
    items_in_stock.insert(3, Furniture::Bed);
    items_in_stock.insert(2, Furniture::Table);
    items_in_stock.insert(0, Furniture::Couch);

    for (k, v) in items_in_stock.iter() {
        if k == &0 {
            println!("Item: {:?}, stock: out of stock", v);
        } else {
            println!("Item: {:?}, stock: {:?}", v, k);
        }

        total_number_of_items += k;
    }

    println!(
        "Total Number of items in stock: {:?}",
        total_number_of_items
    );
}
