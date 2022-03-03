// use serde_json::json;
use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Settings{
    comment_rule: bool,
    fullstop_rule: bool,
    indentation_rule: bool,
    blank_lines_rule: bool,
    blank_lines_amount: usize
}

pub struct JsonHandler {
    json_filename: String
}

impl JsonHandler {
    pub fn new(f : &String) -> JsonHandler {
        return JsonHandler {
            json_filename: f.to_string()
        }
    }
    pub fn read_json(&self) -> (Vec<String>, usize) {
        // read file and send its content through the jsonparser
        let mut file = File::open(&self.json_filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Unable to read file");
        let settings: Settings = serde_json::from_str(&data).expect("JSON was not well-formatted! Tip: Check example on github.");

        // add the rules to be applied in a priority
        let mut rules : Vec<String> = Vec::new();
        if settings.comment_rule {rules.push("comment_rule".to_string())}
        if settings.fullstop_rule {rules.push("fullstop_rule".to_string())}
        if settings.indentation_rule {rules.push("indentation_rule".to_string())}
        if settings.blank_lines_rule {rules.push("blank_lines_rule".to_string())}
        if settings.blank_lines_amount < 1 || settings.blank_lines_amount  > 10 {
            eprintln!("Quitting...The blank lines amount {} is way too damn high!", settings.blank_lines_amount);
            std::process::exit(1)
        }
        return (rules, settings.blank_lines_amount);
    }
}
