# LaTeX-Linter
A simple CLI program which analyses LaTeX files (*.tex)

The default rules are the follwing:
## Comment rule:
  After %there is a space after so 
  ex: 
  
  	  %comment
	becomes:
	  % comment
  
## Fullstop rule:
  After . in a row, split off the row and add the remainder as the next line
  ex
  
  	  hello.alright.fine
	becomes:
	  hello.
      alright.
      fine.
## Blank space rule:
  There are an adjusteble amount of blank lines before a section
  ex:
 
 	  asdaskdla
	  \section{}
	becomes:
	  asdasdad
	
	  \section{}
      
## Indentation rule
  If code is in environment block, all must be indented with two spaces.
  
  	  \begin{}
  	  \item
  	  \end{}
	becomes:
  	  \begin{}
  		  \item
  	  \end{}

