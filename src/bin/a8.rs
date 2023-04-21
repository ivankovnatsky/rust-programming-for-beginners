// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Sparkling,
    Sweet,
}

struct LiquorProperties {
    flavour: Flavour,
    oz: f64,
}

fn print_drink(f: LiquorProperties) {
    match f.flavour {
        Flavour::Sparkling => println!("Sparkling"),
        Flavour::Sweet => println!("Sweet"),
    }

    println!("flavour is {:?} ounces", f.oz);
}

fn main() {
    let tequila = LiquorProperties {
        flavour: Flavour::Sparkling,
        oz: 7.22,
    };

    let whisky = LiquorProperties {
        flavour: Flavour::Sweet,
        oz: 2.22,
    };

    print_drink(tequila);
    print_drink(whisky);
}
