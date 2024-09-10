;; Constants
(integer) @constant

;; Variables and Symbols

(typed_binding (atom (qid) @variable))
(untyped_binding) @variable
(typed_binding (expr) @type)

(id) @function
(bid) @function

(function_name (atom (qid) @function))
(field_name) @function


[(data_name) (record_name)] @constructor

; Set
(SetN) @type


"import"  @include

(module_name) @namespace


(pragma) @constant.macro

(comment) @comment

[
  "where"
  "data"
  "rewrite"
  "postulate"
  "public"
  "private"
  "tactic"
  "Prop"
  "quote"
  "renaming"
  "open"
  "in"
  "hiding"
  "constructor"
  "abstract"
  "let"
  "field"
  "mutual"
  "module"
  "infix"
  "infixl"
  "infixr"
  "record"
  "forall"
  "∀"
  "->"
  "→"
  "\\"
  "λ"
  "..."
  "…"
] @keyword

[
  "("
  ")"
  "{"
  "}"]
@punctuation.bracket
