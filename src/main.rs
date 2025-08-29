use std::collections::HashMap;
use std::io;
mod acronyms;
use crate::acronyms::get_acronyms;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;

fn txt_reader(acronyms: &HashMap<String, String>) {
    let mut file_path = String::new();
    println!("Please enter the file path:");
    io::stdin().read_line(&mut file_path).expect("Failed to read line");
    let file_content = fs::read_to_string(file_path.trim()).expect("Failed to read file");

    let mut input_string = file_content;
    for (acronym, meaning) in acronyms.iter() {
        let pattern = format!(r"\b{}\b", regex::escape(acronym)); //Creates the regex pattern
        let re = Regex::new(&pattern).unwrap(); //Compiles the regex pattern
        input_string = re.replace_all(&input_string, meaning).to_string(); //Replaces the acronym with its meaning
    }

    let mut output_file = File::create("output.txt").expect("Failed to create output file");
    output_file.write_all(input_string.as_bytes()).expect("Failed to write to output file");
}

fn paste_text(acronyms: &HashMap<String, String>) {
    let mut input_string = String::new(); //Creates a new user input variable
    println!("Paste the text to be de-acronymed:"); // Prompt the user for the text
    io::stdin().read_line(&mut input_string).expect("Failed to read line"); //reads the input and looks for errors

    for (acronym, meaning) in acronyms {
        let pattern = format!(r"\b{}\b", regex::escape(acronym)); //Creates the regex pattern
        let re = Regex::new(&pattern).unwrap(); //Compiles the regex pattern
        input_string = re.replace_all(&input_string, meaning).to_string(); //Replaces the acronym with its meaning
    }

    println!("Your acronym free text: {}", input_string.trim()); // Print the de-acronymed text
}

fn menu(acronyms: &HashMap<String, String>) {
    println!("Please select an option:");
    println!("1. Read text from file");
    println!("2. Paste text");
    println!("3. Exit");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");
    match option.trim() {
        "1" => txt_reader(acronyms),
        "2" => paste_text(acronyms),
        "3" => std::process::exit(0),
        _ => println!("Invalid option"),
    }
}

fn main() {
    let acronyms_list = get_acronyms(); // Get the acronyms from the function
    menu(&acronyms_list);
}
