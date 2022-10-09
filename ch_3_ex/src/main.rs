// Chapter 3 Exercises
// ch03-05-control-flow.html

use std::io;

fn f_to_c(temp: i32) -> i32 {
    // (-40°F − 32) × 5/9 = -40°C

    (temp-32) * 5 / 9
}


fn run_ex_1() {
    // Ex1: Convert temperatures between Fahrenheit and Celsius.

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
    // Ex2: Generate the nth Fibonacci number.

    loop {
        println!("Enter which number in the Fibonacci sequence you would like ('q' to exit):");

        let mut seq: String = String::new();
        io::stdin()
            .read_line(&mut seq)
            .expect("Failed to read line");
        
        seq = seq.trim().to_string();
        if seq == "q" {
            break;
        }

        let seq: u32 = match seq.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut i:u32 = 1;
        let mut last:u32 = 0;
        let mut fib:u32 = 1;

        while i < seq {
            let tmp: u32 = fib;
            fib += last;
            last = tmp;
            i += 1;
        }

        println!("{fib}");
    }
}


fn run_ex_3() {
    // Ex3: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
    //      taking advantage of the repetition in the song.

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
