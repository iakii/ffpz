; ── Dart Tree-sitter 语法高亮查询 ──
; 基于 tree-sitter-dart 语法
; 参考：https://github.com/UserNobody14/tree-sitter-dart

; ── 关键字 ──
[
  "abstract"
  "as"
  "assert"
  "async"
  "await"
  "base"
  "break"
  "case"
  "catch"
  "class"
  "const"
  "continue"
  "covariant"
  "default"
  "deferred"
  "do"
  "dynamic"
  "else"
  "enum"
  "export"
  "extends"
  "extension"
  "external"
  "factory"
  "false"
  "final"
  "finally"
  "for"
  "get"
  "hide"
  "if"
  "implements"
  "import"
  "in"
  "interface"
  "is"
  "late"
  "library"
  "mixin"
  "new"
  "null"
  "on"
  "operator"
  "part"
  "required"
  "rethrow"
  "return"
  "sealed"
  "set"
  "show"
  "static"
  "super"
  "switch"
  "sync"
  "this"
  "throw"
  "true"
  "try"
  "typedef"
  "var"
  "void"
  "when"
  "while"
  "with"
  "yield"
] @keyword

; ── 字符串 ──
(string_literal) @string
(interpolation_expression) @embedded
(escape_sequence) @string.escape

; ── 数字 ──
(number_literal) @number

; ── 注释 ──
(line_comment) @comment
(block_comment) @comment
(documentation_comment) @comment.doc

; ── 函数与方法 ──
(function_declaration
  name: (identifier) @function)
(method_declaration
  name: (identifier) @function.method)
(constructor_declaration
  name: (identifier) @constructor)

; ── 参数 ──
(parameters
  (parameter
    name: (identifier) @parameter))
(default_parameter
  name: (identifier) @parameter)

; ── 类 ──
(class_declaration
  name: (identifier) @type)
(mixin_declaration
  name: (identifier) @type)
(enum_declaration
  name: (identifier) @type)
(extension_declaration
  name: (identifier) @type)
(typedef_definition
  name: (identifier) @type)

; ── 属性 ──
(field_declaration
  name: (identifier) @property)
(getter_signature
  name: (identifier) @property)
(setter_signature
  name: (identifier) @property)

; ── 变量 ──
(initialized_variable_declaration
  name: (identifier) @variable)
(final_declaration
  name: (identifier) @variable)

; ── 运算符 ──
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "="
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "&&"
  "||"
  "!"
  "&"
  "|"
  "^"
  "~"
  "<<"
  ">>"
  ">>>"
  "+="
  "-="
  "*="
  "/="
  "%="
  "&="
  "|="
  "^="
  "<<="
  ">>="
  ">>>="
  "??"
  "??="
  "?"
  ":"
  "."
  ".."
  "?."
  "->"
  "=>"
] @operator

; ── 类型注解 ──
(type_identifier) @type
(qualified_type
  (type_identifier) @type)

; ── 元数据注解 ──
(annotation
  "@" @attribute
  name: (identifier) @attribute)
(annotation
  "@" @attribute
  name: (qualified_name) @attribute)

; ── 泛型 ──
(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

; ── 标点符号 ──
[
  ";"
  ","
  "."
] @punctuation.delimiter

; ── 花括号 ──
[
  "{"
  "}"
] @punctuation.bracket

; ── 方括号 ──
[
  "["
  "]"
] @punctuation.bracket

; ── 圆括号 ──
[
  "("
  ")"
] @punctuation.bracket

; ── 标签 ──
(label_declaration
  name: (label) @label)
(label_usage
  name: (label) @label)
