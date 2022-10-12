// Using a hash map and vectors, 
// create a text interface to allow a user to 
// add employee names to a department in a company. 
// For example, “Add Sally to Engineering” 
// or “Add Amir to Sales.” 

// Then let the user retrieve a list of all people in a department 
// or all people in the company by department, sorted alphabetically.


use std::io;
use std::collections::HashMap;


pub fn run() {

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("
Menu:
    [A]dd a new employee
    [L]ist employees
    [Q]uit
Enter Choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        let first_char: char = match choice.trim().chars().nth(0) {
            Some(char) => char,
            None => continue
        };

        match first_char.to_ascii_uppercase() {
            'A' => add_employee(&mut departments),
            'L' => list_employees(&departments),
            'Q' => break,
            _ => continue
        }
    }
}


fn add_employee(departments: &mut HashMap<String, Vec<String>>) {
    let mut name = String::new();
    let mut dept = String::new();

    println!("Employee Name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Department:");

    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read line");

    let emps = 
        departments.entry(dept.trim().to_string())
        .or_insert(vec![]);
    
    emps.push(name.trim().to_string());
    emps.sort();
    println!("{:#?}", departments);
}

use itertools::Itertools;

fn list_employees(departments: &HashMap<String, Vec<String>>) {
    loop {
        println!("\nEnter Department (blank for all):");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        if choice.trim() == "" {
            let keys = departments.keys().sorted();
            for k in keys {
                let v = departments.get(k).unwrap();
                println!("{}: {:?}", k, v);
            }
            break;
        } else {
            let dept = 
                match departments.get(choice.trim()) {
                    Some(v) => v,
                    None => {
                        println!("No Such Department");
                        continue;
                    }
                };

            println!("{:?}", dept);
            break;
        }
    }
}