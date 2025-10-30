; Highlights for Nextflow language (based on Groovy)

[
  "true"
  "false"
] @boolean


(comment) @comment
(shebang) @comment

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
