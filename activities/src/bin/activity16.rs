// Topic: Option

/**
    Program requirements:
    * Print out the details of a student's locker assignment.
    * Lockers use numbers and are optional for students.

    Notes:
    * Use a struct containing the student's name and locker assignment.
    * The locker assignment should use an Option<i32>
**/

// struct for student info
struct Student {
    name: String,
    locker_assignment: Option<i32>
}

fn main() {
    let students_vector = vec![
        Student { name: "Jack".to_owned(), locker_assignment: Some(23) },
        Student { name: String::from("Joe"), locker_assignment: None },
        Student { name: "John".to_owned(), locker_assignment: Some(44) }
    ];

    for student in students_vector {
        println!("{:?}", student.name);
        match student.locker_assignment {
            Some(locker_number) => println!("Locker number: {:?}", locker_number),
            None => println!("No locker assigned.")
        }
    }
}
