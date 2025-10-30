; Highlights for Nextflow language (based on Groovy)
;

(include) @keyword.import
(number) @number
(closure) @closure
(parameter) @parameter

(workflow_definition) @type.definition
(workflow_body) @string
(workflow_input) @type.definition
(workflow_main) @type.definition
(workflow_emit) @type.definition

(process_invocation) @module
(process_output) @type.definition

(command_expression) @variable
(variable_declaration) @keyword.directive

(if_statement) @type.interface
(else_if_clause) @type.interface
(else_clause) @type.interface

(output_declaration) @type.definition
(directive) @keyword.directive
(binary_expression) @operator

[
  "true"
  "false"
] @boolean


(comment) @comment
(shebang) @shebang

(string) @string

("(") @punctuation.bracket
(")") @punctuation.bracket
("[") @punctuation.bracket
("]") @punctuation.bracket
("{") @punctuation.bracket
("}") @punctuation.bracket
(":") @punctuation.delimiter
(",") @punctuation.delimiter
(".") @punctuation.delimiter

(identifier) @variable
((identifier) @variable.parameter
  (#is? @variable.parameter "local.parameter"))

((identifier) @constant
  (#match? @constant "^[A-Z][A-Z_]+"))

[
  "%" "*" "/" "+" "-" ".." "..<" "<"
  "<=" ">" ">=" "==" "!=" "|"
  "&&" "||" "+" "*" ] @operator

(assignment ("=") @operator)
