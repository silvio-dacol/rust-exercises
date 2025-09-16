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
        println!("\nSelect an action from: Add, View, or Quit: ");
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
                quit();
                break;
            }

            _ => {
                println!("Invalid command {}. Please use another command", command)
            }
        }
    }
}

fn add(map: &mut HashMap<String, Vec<String>>) {
    println!("\nWhat's the name of the person you want to add? ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!("\nWhat's the department {name} is working in? ");
    io::stdout().flush().unwrap();
    let mut department = String::new();
    io::stdin().read_line(&mut department).unwrap();
    // Normalize to avoid “Sales” vs “sales” becoming two keys
    let department = department.trim().to_ascii_lowercase();

    // Append or create vector to save the names
    map.entry(department)
        .or_default()
        .push(name);

    println!("{map:?}");
}

// ---------------------------------------- MISSING ---------------------------------------------
fn view(map: &mut HashMap<String, Vec<String>>) {
    loop {
        println!("\nDo you want to visualize the employees in the whole Company or a Department");
        break;
    }

    loop {
        println!("\nWhich Department?");
        break;
    }
}

fn quit() {
    println!("Exiting Application.");
}
