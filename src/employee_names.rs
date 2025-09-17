// Using a hash map and vectors, create a text interface to allow a user to add employee names 
// to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people in the company 
// by department, sorted alphabetically.

use std::collections::HashMap;
use std::io::{self, Write};


pub fn employee_names() {
    // Defining the HashMap that will contain all the employees {"Sales", ["Alberto", "Silvia", "Filippo"]}
    let mut database: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\nSelect an action from: Add, View, or Quit:");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the in-line input");

        // Remove new line
        let command = input.trim().to_ascii_lowercase();

        // Match the user input and find call the correspondent function
        match command.as_str() {
            "add" => {
                add(&mut database);
            }

            "view" => {
                view(&mut database);
            }

            "quit" => {
                println!("Exiting Application");
                break;
            }

            _ => {
                println!("Invalid command {}. Please use another command", command)
            }
        }
    }
}

fn add(map: &mut HashMap<String, Vec<String>>) {
    println!("\nWhat's the name of the person you want to add?");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!("\nWhat's the department {name} is working in?");
    io::stdout().flush().unwrap();

    let mut department = String::new();
    io::stdin().read_line(&mut department).unwrap();

    // Normalize to avoid “Sales” vs “sales” becoming two keys
    let department = department.trim().to_ascii_lowercase();

    // Append or create vector to save the names
    map.entry(department)
        .or_default()
        .push(name);
}

fn view(map: &mut HashMap<String, Vec<String>>) {
    loop {
        println!("\nSelect \"Company\" or a \"Department\" to visualize the employees. Or \"Menu\" to go back.");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the in-line input");

        let command = input.trim().to_ascii_lowercase();

        match command.as_str() {
            "company" => {
                for (department, employee_names) in &*map {
                    // Save the employees in one single string divided by commas
                    let employee_names_string = employee_names.join(", ");

                    // Print the names of the employees
                    println!("The people working in {} are {}", department, employee_names_string);
                }
            }

            "department" => {
                println!("\nWhich Department?");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read the in-line input");

                let department = input.trim().to_ascii_lowercase();

                match map.get(department.as_str()) {
                    Some(employee_names) => {
                        let employee_names_string = employee_names.join(", ");
                        println!("The people working in {} are {}", department, employee_names_string);
                    }

                    _ => {
                        println!("Invalid command {}. Please use another command", command)
                    }
                }
            }

            "menu" => {
                println!("Back to Main Menu");
                break;
            }

            _ => {
                println!("Invalid command {}. Please use another command", command)
            }
        }
    }
}
