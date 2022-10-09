
use std::io;

// Ex1: Convert temperatures between Fahrenheit and Celsius.
// Ex2: Generate the nth Fibonacci number.
// Ex3: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
//      taking advantage of the repetition in the song.

fn f_to_c(temp: i32) -> i32 {
    // (-40°F − 32) × 5/9 = -40°C

    (temp-32) * 5 / 9
}


fn run_ex_1() {
    loop {
        let mut tempf: String = String::new();
        println!("Enter temp in Farenheit ('q' to exit):");
        io::stdin()
            .read_line(&mut tempf)
            .expect("Failed to read line");
        
        tempf = tempf.trim().to_string();
        if tempf == "q" {
            break;
        }

        let tempf: i32 = match tempf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let tempc: i32 = f_to_c(tempf);

        println!("({tempf}°F − 32) × 5/9 = {tempc}°C");
    }
}


fn run_ex_2() {
    println!("Unimplemented")
}


fn run_ex_3() {
    println!("Unimplemented")
}


fn main() {
    loop {
        let mut choice: String = String::new();
        
        println!("
Choose Exercise to Run ('q' to exit):
    [1]: Convert temperatures between Fahrenheit and Celsius.
    [2]: Generate the nth Fibonacci number.
    [3]: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
            taking advantage of the repetition in the song.
Enter Choice:");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        choice = choice.trim().to_string();
        if choice == "q" {
            break;
        } else if choice == "1" {
            run_ex_1();
        } else if choice == "2" {
            run_ex_2();
        } else if choice == "3" {
            run_ex_3();
        }
    }
}
