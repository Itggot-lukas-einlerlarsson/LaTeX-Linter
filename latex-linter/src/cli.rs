mod linter;
use linter::Linter as LaTeXLinter;
mod jsonhandler;
use jsonhandler::JsonHandler;
use std::env;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::io::{Write, BufReader, BufRead};

/// This class takes care of user input and output.

pub struct CLI {
    filename: String,
    arguments : Vec<String>,
    usage_string : String,
}

impl CLI {

    /// Contstructor
    pub fn new() -> CLI {
        return CLI {
            filename:  String::new(),
            arguments: Vec::new(),
            usage_string: "Usage: ./latex-linter infile.tex [rules.json] [ow]".to_string()
        }
    }

    /// This function reads all the commandline arguments and checks if too many were put in.
    pub fn parse_args(&mut self) {
        for arg in env::args().skip(1) {
            self.arguments.push(arg)
        }
        if self.arguments.len() <1 || self.arguments.len() > 3 {
            eprintln!("too few/many arguments. \n{}", self.usage_string);
            std::process::exit(1)
        }
        //check extension:
        self.filename = String::from(&self.arguments[0]);
        let extension = Path::new(&self.filename).extension().and_then(OsStr::to_str);
        if extension != Some("tex") && extension != Some("bib") && extension != Some("tikz") {
            eprintln!("Input file is not valid. Valid files are LaTeX files only('.tex', '.bib', '.tikz').");
            std::process::exit(1)
        }
    }

    /// This function runs the program by calling all the necessary functions. (main running function)
    pub fn run(&self) {
        //read file content
        let content = CLI::read_file(&self);

        //read json content
        let (rules, blank_lines_amount) = CLI::get_settings(&self);

        //creates linter and formats file content
        let mut lonter = LaTeXLinter::new(content, &rules, blank_lines_amount);
        let formatted_content = lonter.format_file();

        //creates file and writes it
        CLI::write_file(self, formatted_content)
    }



    /*                 -----           Private functions           -----                   */

    /// This function first checks if a file exists and then reads it line by line
    fn read_file(&self) -> Vec<String> {
        //checks if file exists
        if Path::new(&self.filename).exists() == false {
            eprintln!("Couldn't locate file: '{}'", self.filename);
            std::process::exit(1);
        }
        //open files and reads it.
        let open_file = File::open(&self.filename).unwrap();
        let reader = BufReader::new(open_file);
        let mut content_lines : Vec<String> = Vec::new();
        for line in reader.lines(){
            content_lines.push(line.unwrap()+ "\n");
        }
        content_lines
    }

    /// This function checks whether default rules or customized rules are going to be applied
    /// if Customized rules are found; fetch them from JsonHandler class and return them.
    fn get_settings(&self) -> (Vec<String>, usize) {
        let rules : Vec<String>;
        let blank_lines_amount : usize = 1;
        if self.arguments.len() == 1 || self.arguments[1].contains(".json") == false {
            //return default arguments in appropriate priority
            rules = vec!["fullstop_rule".to_string(), "comment_rule".to_string(), "indentation_rule".to_string(), "blank_lines_rule".to_string()];
            return (rules, blank_lines_amount);
        }
        if self.arguments.len() > 1 && Path::new(&self.arguments[1]).exists() == false {
            eprintln!("Couldn't locate file: '{}'", self.arguments[1]);
            std::process::exit(1);
        }
        let parser = JsonHandler::new(&self.arguments[1]);
        let (rules, blank_lines_amount) = JsonHandler::read_json(&parser);
        return (rules, blank_lines_amount);
    }

    /// This function checks whether the user want the original file overwritten or not and then writes the outputfile.
    fn write_file(&self, formatted_content : &Vec<String>) {
        let mut original_filename = String::from(&self.filename);
        let outfile : String; 
        if self.arguments[self.arguments.len()-1] == "ow".to_string() {
            outfile = String::from(&self.filename);
        } else {
            let index = original_filename.find('.').unwrap();
            let extension = original_filename.split_off(index);
            outfile = String::from(original_filename + "_copy" + extension.as_str());
        }
        let mut output = File::create(&outfile).expect("Unable to create file.");
        for line in formatted_content  {
            write!(output, "{}" , line).expect("Unable to write to file");
        }
        println!("{} was created in Current working directory.", outfile)
    }
}
