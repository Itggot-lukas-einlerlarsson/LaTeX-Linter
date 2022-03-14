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

/// this class takes care of reading and fetching json file content.

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
        let settings = JsonHandler::read_json_file(&self);

        // get the rules to be applied in a priority from the settings
        let rules = JsonHandler::get_rules(&settings);
        return (rules, settings.blank_lines_amount);
    }

    /// read file and send its content through the jsonparser
    fn read_json_file(&self) -> Settings {
        let mut file = File::open(&self.json_filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Unable to read file");
        let settings: Settings = match serde_json::from_str(&data) {
            Ok(indata) => indata,
            Err(_) => {
                eprintln!("JSON was not well-formatted! Tip: Check example on github.");
                std::process::exit(1)
            }
        };
        settings
    }

    fn get_rules(settings : &Settings) -> Vec<String> {
        let mut rules : Vec<String> = Vec::new();
        if settings.comment_rule {rules.push("comment_rule".to_string())}
        if settings.fullstop_rule {rules.push("fullstop_rule".to_string())}
        if settings.indentation_rule {rules.push("indentation_rule".to_string())}
        if settings.blank_lines_rule {rules.push("blank_lines_rule".to_string())}
        if settings.blank_lines_rule && (settings.blank_lines_amount < 1 || settings.blank_lines_amount  > 10) {
            eprintln!("Quitting...The blank lines amount {} is way too damn high!", settings.blank_lines_amount);
            std::process::exit(1)
        }
        rules
    }
} // end JsonHandler implementation

/*                  -----               Unit tests               -----                   */
#[cfg(test)]
mod tests {
    use crate::cli::jsonhandler::Settings;
    #[test]
    fn test_parser() {
        let data =
        r#"
        {
          "comment_rule": true,
          "fullstop_rule": true,
          "indentation_rule": true,
          "blank_lines_rule": true,
          "blank_lines_amount": 5
        }
        "#;
        let test_settings: Settings = serde_json::from_str(data).expect("json string was wrongkly formatted!");
        assert_eq!(test_settings.comment_rule, true);
        assert_eq!(test_settings.fullstop_rule, true);
        assert_eq!(test_settings.indentation_rule, true);
        assert_eq!(test_settings.blank_lines_rule, true);
        assert_eq!(test_settings.blank_lines_amount, 5);
    }
}
