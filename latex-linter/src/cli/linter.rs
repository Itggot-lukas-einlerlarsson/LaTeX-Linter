mod linter_rules;
use linter_rules::LinterRules;

/// #This struct acts an information expert of the settings, content, options

pub struct Linter {
    contents: Vec<String>,
    rules: Vec<String>,
    blank_lines_amount: usize
}

impl Linter {
    /// Constructor
    pub fn new(c : Vec<String>, r : &Vec<String>, b : usize) -> Linter {
        return Linter{
            contents: c,
            rules: r.to_vec(),
            blank_lines_amount: b
        }
    }

    /// This function loops through the recieved content and formats it according to the rules
    pub fn format_file(&mut self) -> &Vec<String>{
        let mut linter_engine = LinterRules::new();
        for rule in &self.rules {
            match rule.as_str() {
                "comment_rule" => LinterRules::comment_iteration(&mut linter_engine, &mut self.contents),
                "fullstop_rule" => LinterRules::fullstop_iteration(&mut linter_engine, &mut self.contents),
                "blank_lines_rule" => LinterRules::blank_lines_iteration(&linter_engine, &mut self.contents, self.blank_lines_amount),
                "indentation_rule" => LinterRules::indentation_iteration(&linter_engine, &mut self.contents),
                _ => println!("Rule not found! Continuing.")
            }
        }
        &self.contents
    }
}



























/* TODO:
    1. variable for amount of spaces in indentation
    2. variable for holding the settings for some rules - comes from Json file.
    3. kolla över blanklines, den kan inte riktigt see hur många blanklines innan.
    create a match for each rules found in json settings
    for each match iterate through the whole content of the file.

*/
