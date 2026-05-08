; Highlights for Nextflow language (based on Groovy)
;

(include) @keyword.import
(include_item) @emphasis.strong

[
  "include"
  "from"
  "as"
  "process"
  "workflow"
  "params"
  "def"
  "nextflow"
] @keyword

(expression_statement) @expression_statement
(assignment_statement) @expression_statement

(number) @number
(closure) @closure
(parameter) @parameter

(workflow_definition) @type.definition
(workflow_body) @string
(workflow_input) @type.definition
(workflow_main) @type.definition
(workflow_emit) @type.definition

(process_invocation) @module
(process_definition) @type.definition
(process_output) @type.definition

(block) @operator
(input_block) @emphasis.strong
(output_block) @emphasis.strong
(script_block) @emphasis.strong
(directive_block) @emphasis.strong

(directive) @text
(channel_expression) @function.call

(channel_from) @keyword.directive
(channel_value) @keyword.directive
(channel_of) @keyword.directive
(channel_from_list) @keyword.directive

(map) @keyword.directive
(map_entry) @keyword.directive.define
(map_operation) @keyword.operator
(pipe_expression) @keyword.directive

(input_declaration) @type.definition
(output_declaration) @type.definition
(script_content) @text

(command_expression) @variable
(variable_declaration) @keyword.directive

(if_statement) @type.interface
(else_if_clause) @type.interface
(else_clause) @type.interface

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

(feature_flag) @keyword.import
