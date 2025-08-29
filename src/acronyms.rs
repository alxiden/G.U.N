use std::collections::HashMap;
use scraper::{Html, Selector};

pub fn get_acronyms() -> HashMap<String, String> {
    println!("Fetching acronyms from the web...");
    let response = reqwest::blocking::get("https://www.infosecmatter.com/infosec-glossary/")
        .expect("Failed to fetch acronyms");

    let document = Html::parse_document(&response.text().expect("Failed to read response text"));
    let acronym_selector = Selector::parse(".column-1").unwrap();
    let definition_selector = Selector::parse(".column-2").unwrap();

    let acronyms: Vec<_> = document.select(&acronym_selector)
        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
        .collect();

    let definitions: Vec<_> = document.select(&definition_selector)
        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
        .collect();

    let mut map = HashMap::new();
    for (acronym, definition) in acronyms.iter().zip(definitions.iter()) {
        // println!("{}: {}", acronym, definition);
        if acronym == "Acronym" {
            continue; // Skip this entry
        } else {
            map.insert(acronym.clone(), definition.clone());
        }
    }

    println!("Acronyms fetched: {}", map.len());

    map
}
