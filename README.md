# LaTeX-Linter
A simple CLI program which analyses LaTeX files (*.tex)
	
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
  There are an adjusteble amount of blank lines before a section
  ex:
 
 	  asdaskdla
	  \section{}
	becomes:
	  asdasdad
	
	  \section{}
      
## Indentation rule
  If code is in environment block, all data are be indented with two spaces. \begin{document} is an exception
  
  	  \begin{}
  	  \item
  	  \end{}
	becomes:
  	  \begin{}
  	    \item
  	  \end{}


## Json file format - ex:

	{
 	  "comment_rule": true,
	  "fullstop_rule": true,
	  "indentation_rule": false,
 	  "blank_lines_rule": false,
	  "blank_lines_amount": 5
	}

