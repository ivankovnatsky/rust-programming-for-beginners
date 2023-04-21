// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn get_state(s: PowerState) {
    match s {
        PowerState::Off => println!("Computer will be shut off"),
        PowerState::Sleep => println!("Computer will go sleep"),
        PowerState::Reboot => println!("Computer will be rebooted"),
        PowerState::Shutdown => println!("Computer will be shutdown"),
        PowerState::Hibernate => println!("Computer will hibernate"),
    }
}

fn main() {
    let mut input_string = String::new();

    match get_input() {
        Ok(s) => input_string = s.to_lowercase(),
        Err(e) => println!("The error occured: {:?}", e),
    }

    let power_state = match input_string.as_str() {
        "off" => PowerState::Off,
        "sleep" => PowerState::Sleep,
        "reboot" => PowerState::Reboot,
        "shutdown" => PowerState::Shutdown,
        "hibernate" => PowerState::Hibernate,
        _ => panic!("Unexpected power state: {}", input_string),
    };

    get_state(power_state);
}
