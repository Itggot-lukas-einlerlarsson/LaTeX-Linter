/// This class acts as a Linter-Engine, doing all the formatting.

pub struct LinterRules {
    error_counter : usize
}

impl LinterRules {
    pub fn new() -> LinterRules {
        return LinterRules {
            error_counter : 0
        }
    }

    pub fn comment_iteration(&mut self, contents : &mut Vec<String>) {
        // loop - variables:
        let mut line_nr = 0;
        // Loop - applying rules
        while line_nr < contents.len() {
            if LinterRules::comment_rule(&mut contents[line_nr]) {
                self.error_counter += 1
            }
            line_nr += 1
        }
    }

    pub fn fullstop_iteration(&mut self, contents : &mut Vec<String>){
        // loop - variables:
        let mut line_nr = 0;
        // Loop - applying rules
        while line_nr < contents.len() {
            if contents[line_nr].contains('.') == false {
                line_nr += 1;
                continue;
            } else {
                LinterRules::fullstop_rule(&mut contents[line_nr]);
                self.error_counter += 1;
            }
            line_nr += 1
        }
    }

    pub fn indentation_iteration(&mut self, contents : &mut Vec<String>){
        // loop - variables:
        let mut line_nr = 0;
        let mut indentation_amount = 0;
        // Loop - applying rules
        while line_nr < contents.len() {
            LinterRules::remove_indentation(&mut contents[line_nr]);
            LinterRules::indentation_rule(self, &mut contents[line_nr], &mut indentation_amount);
            line_nr += 1
        }
    }

    pub fn blank_lines_iteration(&mut self, contents : &mut Vec<String>, amount : usize){
        // loop - variables:
        let mut line_nr = 1;
        // Loop - applying rules
        while line_nr < contents.len() {
            // check if it applies:
            if LinterRules::blank_line_rule(&contents[line_nr]) {
                self.error_counter += 1;
                LinterRules::add_blank_lines(contents, &mut line_nr, amount)
            }
            line_nr += 1
        }
    }

    pub fn get_error_amount(&self) -> usize{
        self.error_counter
    }


    /*                 -----           Private functions           -----                   */

    /// This rule adds a space after a comment symbol '%' and returns true if comment was found
    fn comment_rule(line : &mut String) -> bool {
        if line.contains("% ") == true {
            return false;
        }
        if line.contains('%') {
            if line.contains("% ") {
                return false;
            } else {
                let new_line = line.replacen('%', "% ", 1);
                *line = new_line;
                return true;
            }
        }
        false
    }

    /// This function deals with fullstops
    /// dotdotdots... and numbers like 20,123,203.123 are exceptions
    fn fullstop_rule(line : &mut String){
        if line.starts_with('%') {
            return
        }
        if line.contains('%') {
            let index = line.find('%').unwrap_or(line.len());
            let mut rest_str : String;
            if line.contains("% ") {
                rest_str = line.split_off(index);
            } else {
                rest_str = line.split_off(index);
                rest_str.insert(1, ' ');
            }
            let temp = line.replace(". ", ".\n");
            *line = String::from(temp.to_string() + &rest_str);
            return
        }
        let temp = line.replace(". ", ".\n");
        *line = String::from(&temp);
    }

    /// This function checks if we are in an environment block
    fn indentation_rule(&mut self, line : &mut String, indentation_amount : &mut usize) {
        //indentation lists:
        let black_list = vec!["begin{document"]; //exception
        let white_list_decrease = vec!["end{"]; //decrease indentation amount
        let white_list_increase = vec!["begin{"]; //increase indentation amount
        for end_str in white_list_decrease {
            if line.contains(end_str){
                if *indentation_amount > 0 {
                    *indentation_amount -= 1;
                }
            }
        }
        LinterRules::indent(self, line, *indentation_amount);
        for exception_str in black_list {
            if line.contains(exception_str) {
                return
            }
        }
        for begin_str in white_list_increase {
            if line.contains(begin_str) {
                *indentation_amount += 1;
            }
        }
    }

    /// This function adds an indentation block if the line is in an environment block
    fn indent(&mut self, line : &mut String, amount : usize){
        if amount > 1 {
            self.error_counter += 1;
        }
        let indent_str = "    ";
        let mut indentation : String = String::new();
        let mut count = 0;
        while count < amount {
            indentation.push_str(indent_str);
            count += 1
        }
        if line.contains(".\n") {
            line.insert_str(0, &indentation);
            let mut temp = line.replace("\n", &("\n".to_string() + &indentation));
            temp = temp.trim_end_matches(' ').to_string();
            *line = temp;
        } else {
            line.insert_str(0, &indentation);
        }
    }

    ///This function checks if new section/chapter etc is found
    fn blank_line_rule(line : &String) -> bool {
        let mut apply_blank_lines = false;
        let white_list : Vec<&str> = vec!["section{", "section*", "subsection{", "subsection*","subsubsection{", "subsubsection*"];
        for section_str in white_list {
            if line.contains(section_str) {
                apply_blank_lines = true;
            }
        }
        return apply_blank_lines;
    }

    /// This function checks how many blanklines before and adds the rest.
    fn add_blank_lines(contents : &mut Vec<String>, line_nr : &mut usize, amount_of_newlines : usize) {
        let mut current_amount = 0;
        let mut index = *line_nr-1;
        let mut count = 0;
        // check how many blanklines before:
        while count < amount_of_newlines && index > 1 {
            if contents[index] == "\r\n" || contents[index] == "\n" || contents[index].as_str().ends_with("\r\n\n"){
                current_amount += 1;
            }
            index -= 1;
            count += 1;
        }
        // add rest of blanklines (if there is any)
        while current_amount < amount_of_newlines {
                contents.insert(*line_nr, "\n".to_string());
                *line_nr += 1;
                current_amount+= 1
        }
    }

    /// This function removes all indentation of a line.
    fn remove_indentation(line : &mut String) {
        let new_line = line.trim_start_matches(' ').to_string();
        *line = new_line.to_string();
    }
} // end implementation
