// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

fn can_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    } else {
        Err("Customer can't be verified to allow this purchase".to_owned())
    }
}

fn main() {
    let customers = vec![Customer { age: 32 }, Customer { age: 18 }];

    for i in customers {
        let p = can_purchase(&i);
        println!("{:?}", p)
    }
}
