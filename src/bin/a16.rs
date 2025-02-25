// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let s = Student {
        name: "Brok".to_owned(),
        locker: Some(237),
    };

    println!("Student's name: {:?}", s.name);

    match s.locker {
        Some(la) => println!("Student's locker assignment: {:?}", la),
        None => (),
    }
}
