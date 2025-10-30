; Highlights for Nextflow language (based on Groovy)
;

(include) @keyword.import

(number) @number
(closure) @closure
(workflow_definition) @type.definition
(output_declaration) @type.definition
(directive) @keyword.directive
(binary_expression) @operator
(process_invocation) @module
(parameter) @parameter
(include) @keyword.directive
(variable_declaration) @keyword.directive


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
