mod linter_rules;
use linter_rules::LinterRules;

/// #This class store the content and the settings of the LaTeX file but lets the LinterRules class act as an engine for the formatting.

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
    /// it returns the new formatted content along with the amount of errors found and fixed.
    pub fn format_file(&mut self) -> (&Vec<String>, usize) {
        let mut linter_engine = LinterRules::new();
        for rule in &self.rules {
            match rule.as_str() {
                "comment_rule" => LinterRules::comment_iteration(&mut linter_engine, &mut self.contents),
                "fullstop_rule" => LinterRules::fullstop_iteration(&mut linter_engine, &mut self.contents),
                "blank_lines_rule" => LinterRules::blank_lines_iteration(&mut linter_engine, &mut self.contents, self.blank_lines_amount),
                "indentation_rule" => LinterRules::indentation_iteration(&mut linter_engine, &mut self.contents),
                _ => println!("Rule not found! Continuing.")
            }
        }
        (&self.contents, LinterRules::get_error_amount(&linter_engine))
    }
}
