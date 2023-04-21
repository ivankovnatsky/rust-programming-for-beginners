// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still status
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Department {
    Maintenance,
    Marketing,
    Managers,
    Supervisors,
    Kitchen,
    Assembly,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    department: Department,
    status: Status,
}

fn print_access(e: &Employee) -> Result<(), String> {
    can_access_building(&e)?;
    println!("Access granted");
    Ok(())
}

fn can_access_building(e: &Employee) -> Result<(), String> {
    match e.status {
        Status::Terminated => return Err("Employee is terminated".to_owned()),
        _ => (),
    }

    match e.department {
        Department::Maintenance => Ok(()),
        Department::Marketing => Ok(()),
        Department::Managers => Ok(()),
        _ => Err("Employee can't enter the building".to_owned()),
    }
}

fn main() {
    let employees = vec![
        Employee {
            department: Department::Maintenance,
            status: Status::Active,
        },
        Employee {
            department: Department::Marketing,
            status: Status::Active,
        },
        Employee {
            department: Department::Assembly,
            status: Status::Terminated,
        },
        Employee {
            department: Department::Managers,
            status: Status::Active,
        },
        Employee {
            department: Department::Kitchen,
            status: Status::Terminated,
        },
        Employee {
            department: Department::Supervisors,
            status: Status::Terminated,
        },
    ];

    for e in employees {
        match print_access(&e) {
            Err(e) => println!("Access denied: {:?}", e),
            _ => (),
        }
    }
}
