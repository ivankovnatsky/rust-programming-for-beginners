// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::collections::HashMap;
use std::io;
use std::io::Write;

fn get_input(item: String) -> io::Result<String> {
    let mut buffer = String::new();

    match item.as_str() {
        "name" => print!("Enter the name of the bill: "),
        "amount" => print!("Enter the amount of the bill: "),
        "action" => print!("Choose the action: "),
        _ => panic!("You can only use `name` or `amount` types."),
    }

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn get_string(result: String) -> String {
    let s = get_input(result);

    match s {
        Ok(s) => s,
        Err(e) => panic!("Something wrong happened: {:?}", e),
    }
}

fn get_action() -> String {
    let s = get_input("action".to_owned());

    match s {
        Ok(s) => s.to_lowercase(),
        Err(e) => panic!("Something wrong happened: {:?}", e),
    }
}

fn show_menu() {
    println!(
        r#"
This application gives an ability to manipulate with the bills, choose one of the following
actions:

* Add
* View
* Remove
* Edit
* Back

You can type in any case.
        "#
    )
}

fn add_bills(bills: &mut HashMap<String, f32>, name: String, amount: f32) -> &HashMap<String, f32> {
    bills.insert(name, amount);
    bills
}

fn remove_bills(bills: &mut HashMap<String, f32>, name: String) -> &HashMap<String, f32> {
    if bills.contains_key(&name) {
        bills.remove(&name);
    } else {
        println!("There's no such item in bills.")
    }

    bills
}

fn main() {
    let mut bills: HashMap<String, f32> = HashMap::new();

    show_menu();

    loop {
        let action = get_action();

        match action.as_str() {
            "menu" => {
                show_menu();
            }
            "add" => {
                add_bills(
                    &mut bills,
                    get_string("name".to_owned()),
                    get_string("amount".to_owned()).parse::<f32>().unwrap(),
                );
            }
            "view" => {
                println!("Bills content is: {:?}", bills);
            }
            "remove" => {
                remove_bills(&mut bills, get_string("name".to_owned()));
            }
            "edit" => {
                println!("Choose the bill you want to update following the amount");
                add_bills(
                    &mut bills,
                    get_string("name".to_owned()),
                    get_string("amount".to_owned()).parse::<f32>().unwrap(),
                );
            }
            "back" => {
                bills.clear();
            }
            _ => {
                show_menu();
            }
        }
    }
}
