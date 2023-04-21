// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

const FILE_NAME: &str = "src/bin/p2_data.csv";

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("Could not read the name entered.")]
    CantReadNameInput,
    #[error("Could not read the email entered.")]
    CantReadEmailInput,
    #[error("Wrong email entered, verify if you have entered the @ symbol.")]
    InvalidNameInput,
    #[error("Could not read the record id")]
    InvalidRecordId,
    #[error("This record is not found in the database")]
    RecordNotFound,
}

#[derive(Debug)]
struct Contact {
    id: i32,
    name: String,
    email: Option<String>,
}

#[derive(Debug)]
struct Database {
    database: HashMap<i32, Contact>,
}

impl Database {
    fn new() -> Self {
        Self {
            database: HashMap::new(),
        }
    }

    fn view_db(self) {
        let db = self.into_vec();
        for item in db {
            println!("{:?}", item);
        }
    }

    fn into_vec(mut self) -> Vec<Contact> {
        let mut db: Vec<_> = self.database.drain().map(|kv| kv.1).collect();
        db.sort_by_key(|rec| rec.id);
        db
    }

    fn add_to_db(mut self) -> Result<Self, Error> {
        let mut ids: Vec<_> = self.database.keys().collect();
        ids.sort();
        let max_id = ids.last().unwrap_or(&&0).clone();

        print!("Enter the name of the contact: ");
        let name = match get_input() {
            Some(input) => input,
            None => return Err(Error::CantReadNameInput),
        };

        print!("Enter email of the contact: ");
        let email = match get_input() {
            Some(input) => {
                if input.contains("@") {
                    input
                } else {
                    return Err(Error::InvalidNameInput);
                }
            }
            None => return Err(Error::CantReadEmailInput),
        };

        self.database.insert(max_id + 1, Contact {
            id: max_id + 1,
            name,
            email: Some(email),
        });

        Ok(self)
    }

    fn search_db(self) {
        print!("Choose the search pattern: name/email: ");
        let search_pattern = match get_input() {
            Some(input) => match input.as_str() {
                "name" => input,
                "email" => input,
                _ => return,
            },
            None => return,
        };

        print!("Enter search value to look up: ");
        let search_value = match get_input() {
            Some(input) => input.to_lowercase(),
            None => return,
        };

        match search_pattern.to_lowercase().as_str() {
            "name" => {
                println!("Search results:");
                let mut contacts: Vec<&Contact> = self
                    .database
                    .values()
                    .filter(|contact| contact.name.to_lowercase().contains(search_value.as_str()))
                    .collect();
                contacts.sort_by_key(|contact| contact.id);
                for contact in contacts {
                    println!("{:?}", contact);
                }
            }
            "email" => {
                println!("Search results:");
                let mut contacts: Vec<&Contact> = self
                    .database
                    .values()
                    .filter(|contact| {
                        contact
                            .email
                            .as_ref()
                            .map(|s| s.to_lowercase().contains(search_value.as_str()))
                            .unwrap_or(false)
                    })
                    .collect();
                contacts.sort_by_key(|contact| contact.id);
                for contact in contacts {
                    println!("{:?}", contact);
                }
            }
            _ => println!("Not searching, something went wrong."),
        }
    }

    fn edit_db(mut self) -> Result<Self, Error> {
        print!("Choose the record to update: ");
        let id_input = match get_input() {
            Some(input) => input,
            None => return Err(Error::InvalidRecordId),
        };
        let id_input_int = id_input.parse::<i32>().unwrap();

        print!("Enter name of the contact you want to update: ");
        let name = match get_input() {
            Some(input) => input,
            None => return Err(Error::CantReadNameInput),
        };

        print!("Enter email of the contact you want to update: ");
        let email = match get_input() {
            Some(input) => input,
            None => return Err(Error::CantReadEmailInput),
        };

        self.database.insert(
            id_input_int,
            Contact {
                id: id_input_int,
                name: name.clone(),
                email: Some(email.clone()),
            },
        );

        Ok(self)
    }

    fn remove_from_db(mut self) -> Result<Self, Error> {
        print!("Choose the record to remove: ");
        let id_input = match get_input() {
            Some(input) => input,
            None => return Err(Error::InvalidRecordId),
        };
        let id_input_int = id_input.parse::<i32>().unwrap();

        match self.database.remove(&id_input_int) {
            Some(_) => Ok(self),
            None => Err(Error::RecordNotFound),
        }
    }
}

fn save_db(contents: String) -> std::io::Result<()> {
    let mut file = File::create(FILE_NAME)?;
    let contents_bytes = contents.as_bytes();
    file.write_all(contents_bytes)?;
    file.flush()?;
    Ok(())
}

fn convert_to_csv(db: Database) -> String {
    let mut db_string = String::new();
    db_string += &format!(
        "{},{},{}\n",
        "id".to_string(),
        "name".to_string(),
        "email".to_string()
    );

    let db_vec_data = db.into_vec();
    for i in db_vec_data {
        if let Some(email) = i.email {
            db_string += &format!("{},{},{}\n", i.id, i.name, email);
        }
    }

    db_string
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(filename: &str) -> Vec<Vec<String>> {
    let mut db = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                let record: Vec<String> = l.split(",").map(|s| s.to_string()).collect();
                db.push(record);
            }
        }
    }

    db
}

fn remove_wrong_records(filename: &str) -> Vec<Vec<String>> {
    let mut db: Vec<Vec<String>> = Vec::new();
    let records = read_file(filename);

    for record in records {
        match record.first() {
            Some(string) => match string.parse::<i32>() {
                Ok(..) => {
                    db.push(record);
                }
                Err(..) => (),
            },
            None => (),
        }
    }

    db
}

fn clear_data(filename: &str) -> Database {
    let mut db = Database::new();
    let db_raw = remove_wrong_records(filename);

    for record in db_raw {
        if let (Some(id_str), Some(name), email) = (record.get(0), record.get(1), record.get(2)) {
            if let Ok(id) = id_str.parse::<i32>() {
                db.database.insert(id,
                    Contact {
                    id,
                    name: name.to_string(),
                    email: email.cloned(),
                });
            }
        }
    }

    db
}

fn show_menu() {
    println!(
        r#"
This application gives an ability to manipulate with contacts. To interact with the app choose the
following actions:

* View
* Add
* Search
* Edit
* Remove

You can type in any case.
        "#
    )
}

/// Retrieves user input. This function will automatically retry on
/// io errors, and will return None if the user did not enter any data.
fn get_input() -> Option<String> {
    let mut buffer = String::new();

    io::stdout().flush().unwrap();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
    }

    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

#[derive(Debug)]
enum Action {
    View,
    Add,
    Search,
    Edit,
    Remove,
    Quit,
}

impl From<String> for Action {
    fn from(input: String) -> Self {
        match input.to_lowercase().as_str() {
            "view" => Self::View,
            "add" => Self::Add,
            "search" => Self::Search,
            "edit" => Self::Edit,
            "remove" => Self::Remove,
            _ => Self::Quit,
        }
    }
}

fn flow() {
    loop {
        show_menu();

        print!("Enter your action: ");
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };

        let action = Action::from(input);
        match action {
            Action::View => {
                let db = clear_data(FILE_NAME);
                Database::view_db(db);
            }
            Action::Add => {
                let db = clear_data(FILE_NAME);
                match Database::add_to_db(db) {
                    Ok(r) => {
                        let contents = convert_to_csv(r);
                        match save_db(contents) {
                            Ok(_r) => println!("Contact added."),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e)
                    }
                };
            }
            Action::Search => {
                let db = clear_data(FILE_NAME);
                Database::search_db(db)
            }
            Action::Edit => {
                let db = clear_data(FILE_NAME);
                match Database::edit_db(db) {
                    Ok(r) => {
                        let contents = convert_to_csv(r);
                        match save_db(contents) {
                            Ok(_r) => println!("Contact updated."),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e)
                    }
                };
            }
            Action::Remove => {
                let db = clear_data(FILE_NAME);
                match Database::remove_from_db(db) {
                    Ok(r) => {
                        let contents = convert_to_csv(r);
                        match save_db(contents) {
                            Ok(_r) => println!("Contact removed."),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e)
                    }
                };
            }
            Action::Quit => break,
        }
    }
}

fn main() {
    flow();
}
