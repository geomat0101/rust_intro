
// Convert strings to pig latin. The first consonant of each word 
//     is moved to the end of the word and “ay” is added, so “first” 
//     becomes “irst-fay.” Words that start with a vowel have “hay” 
//     added to the end instead (“apple” becomes “apple-hay”). 
//     Keep in mind the details about UTF-8 encoding!


use std::io;

fn is_vowel (letter: char) -> bool {
    let vowel = match letter {
        'A' => true,
        'E' => true,
        'I' => true,
        'O' => true,
        'U' => true,
        _ => false
    };
    vowel
}

pub fn run() {
    println!("Enter a phrase in English:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut output = String::new();

    for word in input.trim().split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        let other_chars = chars.as_str();

        if is_vowel(first_char.to_ascii_uppercase()) {
            output.push_str(format!(" {}-hay", word).as_str());
        } else {
            output.push_str(
                format!(
                    " {}-{}ay", 
                    other_chars, 
                    first_char
                ).as_str()
            );
        }
    }
    println!("{}", output.trim())
}
