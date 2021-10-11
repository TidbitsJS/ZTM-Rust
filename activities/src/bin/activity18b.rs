// Topic: Result & the question mark operator

/**
    Program requirements:
    * Determine if an employee can access a building using a digital keycard.
    * Employees that can access the building are:
        * Maintenace crews
        * Marketing department employees
        * Managers
    * Other employees that work at the company are:
        * Line supervisors
        * Kitchen staff
        * Assembly technicians
    * Ensure that terminated employees cannot access the building.
        reagrdless of their position.

    Notes:
    * Use an enum to represent all types of employees
    * Use a struct to store the employee type and whether they are still employed
    * Use a function that returns a Result to determine if the employee may enter the building
    * Print whether the employee may access the building or not
    * Must use a function that utilizes the question mark operator to do this
**/

#[derive(Debug)]
enum EmployeeType {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

#[derive(Debug)]
struct Employee {
    name: String,
    employee_type: EmployeeType,
    is_employed: bool,
}

fn has_access(employee: &Employee) -> Result<(), String> {
    if employee.is_employed {
        match employee.employee_type {
            EmployeeType::Maintenance => Ok(()),
            EmployeeType::Marketing => Ok(()),
            EmployeeType::Manager => Ok(()),
            _ => Err("Employee is not allowed to enter the building".to_owned()),
        }
    } else {
       return Err("Employee is terminated".to_owned())
    }
}

fn print_choice(employee: &Employee) -> Result<(), String> {
    let result = has_access(employee)?;
    Ok(())
}

fn main() {
    let sam = Employee {
        name: "John Doe".to_owned(),
        employee_type: EmployeeType::AssemblyTechnician,
        is_employed: true,
    };

    match print_choice(&sam) {
        Ok(()) => println!("Sam may enter the building"),
        Err(e) => println!("{:?}", e),
    }
}
