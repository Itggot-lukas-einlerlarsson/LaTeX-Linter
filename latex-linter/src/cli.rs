mod linter;
use linter::Linter as LaTeXLinter;
mod jsonhandler;
use jsonhandler::JsonHandler;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::io::{Write, BufReader, BufRead};
use clap::Parser;

/// LaTeXLinter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to format
    #[clap(short, long)]
    file: String,

    /// add Customized settings with json file or use 'default' settings
    #[clap(short, long)]
    json_file: String,

    /// Overwrite option
    #[clap(short, long)]
    overwrite: bool,
}
impl Default for Args {
    fn default () -> Args {
        return Args{file : "nonexistant.file".to_string(), json_file : "default".to_string(), overwrite : false}
    }
}

/// This class takes care of user input and output.

pub struct CLI {
    args : Args,
}

impl CLI {

    /// Contstructor
    pub fn new() -> CLI {
        return CLI {
            args: Args::default(),
        }
    }

    /// This function reads all the commandline arguments and checks if too many were put in.
    pub fn parse_args(&mut self) {
        self.args = Args::parse();
        //check extension:
        let extension = Path::new(&self.args.file).extension().and_then(OsStr::to_str);
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
        let (formatted_content, error_amount) = lonter.format_file();

        //prints error amount found
        println!("found and fixed {} errors.", error_amount);

        //creates file and writes the formatted content to file
        CLI::write_file(self, formatted_content)
    }



    /*                 -----           Private functions           -----                   */

    /// This function first checks if a file exists and then reads it line by line
    fn read_file(&self) -> Vec<String> {
        //checks if file exists
        if Path::new(&self.args.file).exists() == false {
            eprintln!("Couldn't locate file: '{}'", self.args.file);
            std::process::exit(1);
        }
        //open files and reads it.
        let open_file = File::open(&self.args.file).unwrap();
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
        if self.args.json_file == "default" {
            //return default arguments in appropriate priority
            rules = vec!["fullstop_rule".to_string(), "comment_rule".to_string(), "indentation_rule".to_string(), "blank_lines_rule".to_string()];
            return (rules, blank_lines_amount);
        }
        if self.args.json_file != "default" && (self.args.json_file.contains(".json") == false || Path::new(&self.args.json_file).exists() == false) {
            eprintln!("Couldn't locate or fetch data from json-file: '{}'", self.args.json_file);
            std::process::exit(1);
        }
        let parser = JsonHandler::new(&self.args.json_file);
        let (rules, blank_lines_amount) = JsonHandler::read_json(&parser);
        return (rules, blank_lines_amount);
    }

    /// This function checks whether the user want the original file overwritten or not and then writes the outputfile.
    fn write_file(&self, formatted_content : &Vec<String>) {
        let mut original_filename = String::from(&self.args.file);
        let outfile : String;
        if self.args.overwrite {
            outfile = String::from(&self.args.file);
        } else {
            let index = original_filename.find('.').unwrap();
            let extension = original_filename.split_off(index);
            outfile = String::from(original_filename + "_copy" + extension.as_str());
        }
        let mut output = File::create(&outfile).expect("Unable to create file.");
        for line in formatted_content  {
            write!(output, "{}" , line).expect("Unable to write to file");
        }
        println!("  -> {} was created in Current working directory.", outfile)
    }
} // end CLI implementation
