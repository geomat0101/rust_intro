
use std::io;

// Ex1: Convert temperatures between Fahrenheit and Celsius.
// Ex2: Generate the nth Fibonacci number.
// Ex3: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
//      taking advantage of the repetition in the song.

fn f_to_c(temp: i32) -> i32 {
    // (-40°F − 32) × 5/9 = -40°C

    (temp-32) * 5 / 9
}


fn main() {
    loop {
        let mut tempf = String::new();
        println!("Enter temp in Farenheit:");
        io::stdin()
            .read_line(&mut tempf)
            .expect("Failed to read line");

        let tempf: i32 = match tempf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let tempc: i32 = f_to_c(tempf);

        println!("({tempf}°F − 32) × 5/9 = {tempc}°C");
    }
}
