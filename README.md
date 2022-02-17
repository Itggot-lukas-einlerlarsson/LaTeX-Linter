# LaTeX-Linter
A simple CLI program which analyses LaTeX files (*.tex)

usage is : ./latex-linter infile.tex [custom_rules.json] [ow]

[] = optional

ow = overwrite

## Json file format:
[Json file example here with settings and rules]


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
	  
# (Optional extra rules)

