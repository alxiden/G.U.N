use std::io;
mod acronyms;
use crate::acronyms::get_acronyms;
use regex::Regex;

fn main() {
    let acronyms = get_acronyms(); // Get the acronyms from the function
    let mut input_string = String::new(); //Creates a new user input variable
    println!("Paste the text to be de-acronymed:"); // Prompt the user for the text
    io::stdin().read_line(&mut input_string).expect("Failed to read line"); //reads the input and looks for errors

    for (acronym, meaning) in &acronyms {
        let pattern = format!(r"\b{}\b", regex::escape(acronym)); //Creates the regex pattern
        let re = Regex::new(&pattern).unwrap(); //Compiles the regex pattern
        input_string = re.replace_all(&input_string, *meaning).to_string(); //Replaces the acronym with its meaning
    }

    println!("Your acronym free text: {}", input_string.trim()); // Print the de-acronymed text
}
