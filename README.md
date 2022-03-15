# LaTeX-Linter
This is a simple CLI program that analyses LaTeX files (*.tex, *.bib, *.tikz) and formats a copy (or overwrites if user wants)

I coded this program when i studied software engineering(course PA1456 on BTH).
	
	latex-linter 1.4.0
	LaTeXLinter

	USAGE:
    latex-linter.exe [OPTIONS] --file <FILE> --json-file <JSON_FILE>

	OPTIONS:
    -f, --file <FILE>              Name of the file to format
    -h, --help                     Print help information
    -j, --json-file <JSON_FILE>    add Customized or use 'default' settings
    -o, --overwrite                Overwrite the inputfile with formatted file
    -V, --version                  Print version information

# Default rules 
These rules are all applied under the '-j default' option. If you want to customize the formatting, read under the json example
## Comment rule:
  After %there is a space after so 
  ex: 
  
  	  %comment
	becomes:
	  % comment
  
## Fullstop rule:
  After . in a row, split off the row and add the remainder as the next line
  ex
  
  	  volleyball....basketball. wienerdog. SPORTS.
	becomes:
	  volleyball....
      basketball.
      wienerdog.
      SPORTS.
## Blank space rule:
  There are an adjusteble amount of blank lines before a section (default is 1 blank line)
  ex:
 
 	  asdaskdla
	  \section{}
	becomes:
	  asdasdad
	
	  \section{}
      
## Indentation rule
  If code is in environment block, all data are be indented. \begin{document} is an exception
  
  	  \begin{}
  	  \item
  	  \end{}
	becomes:
  	  \begin{}
  	    \item
  	  \end{}


## Customize format with Json file - example:

	{
 	  "comment_rule": true,
	  "fullstop_rule": true,
	  "indentation_rule": false,
 	  "blank_lines_rule": false,
	  "blank_lines_amount": 5
	}

	  
# Installation
You can either just download the source code and compile yourself with rust, use in directory:

	'cargo build --release' 
	
The program is then placed in the CWD/target/release/ directory. You can also download releases for Windows and Linux under the releases tab.

# Bill of Materials (dependencies)
You can find all information about the third party components of my program(version, github etc) in the Cargo.toml file. 
