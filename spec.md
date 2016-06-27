# Interpreter overview

1. Source
  - Read from stdin (assumed UTF-8)

2. Lexical analysis
  - Divide source into token sequence
  - Replace special tokens with other tokens

3. Syntax analysis
  - Parse token sequence into syntax tree

4. Semantic analysis
  - Traverse syntax trees
  - Declare variables
  - Load symbol tables
  - Assign types
  - Determine program meaning

5. Code execution
  - Code is evaluated (as everything is an expression) and outputted to stdout
