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

    pub fn indentation_iteration(&self, contents : &mut Vec<String>){
        // loop - variables:
        let mut line_nr = 0;
        let mut indentation_amount = 0;
        // Loop - applying rules
        while line_nr < contents.len() {
            LinterRules::remove_indentation(&mut contents[line_nr]);
            LinterRules::indentation_rule(&mut contents[line_nr], &mut indentation_amount);
            line_nr += 1
        }
    }

    pub fn blank_lines_iteration(&self, contents : &mut Vec<String>, amount : usize){
        // loop - variables:
        let mut line_nr = 1;
        // Loop - applying rules
        while line_nr < contents.len() {
            // check if it applies:
            if LinterRules::blank_line_rule(&contents[line_nr]) {
                LinterRules::add_blank_lines(contents, &mut line_nr, amount)
            }
            line_nr += 1
        }
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
        let temp = line.replace(". ", ".\n");
        *line = String::from(&temp);
    }

    /// This function checks if we are in an environment block
    fn indentation_rule(line : &mut String, indentation_amount : &mut usize) {
        if line.contains("end{"){
            if *indentation_amount > 0 {
                *indentation_amount -= 1;
            }
        }
        //indent
        LinterRules::indent(line, *indentation_amount);
        //if an indention exception
        if line.contains("begin{document") {
            return
        }
        if line.contains("begin{") {
            *indentation_amount += 1;
        }
    }

    /// This function adds an indentation block if the line is in an environment block
    fn indent(line : &mut String, amount : usize){
        //indentation variable
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

    ///This function adds blank lines to previous lines if it starts a new section
    /// returns if new section/chapter etc is found
    fn blank_line_rule(line : &String) -> bool {
        if line.contains("section{"){
             return true;
        } else if line.contains("section*"){
             return true;
        } else if line.contains("subsection{"){
             return true;
        } else if line.contains("subsection*"){
             return true;
        } else if line.contains("subsubsection{"){
             return true;
        } else if line.contains("subsubsection*"){
             return true;
        } else {
            return false
        }
    }

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
        // add rest of blanklines
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
