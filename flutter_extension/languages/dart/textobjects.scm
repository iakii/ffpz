; ── Dart Vim 文本对象 ──

; 函数/方法（around + inside）
(function_declaration
  body: (function_body) @function.around)
(method_declaration
  body: (block) @function.around)
(getter_signature
  body: (function_body) @function.around)
(setter_signature
  body: (function_body) @function.around)
(constructor_declaration
  body: (function_body) @function.around)

; 函数/方法内部
(block) @function.inside

; 类定义（around + inside）
(class_declaration
  body: (class_body) @class.around)
(mixin_declaration
  body: (mixin_body) @class.around)
(enum_declaration
  body: (enum_body) @class.around)

(class_body) @class.inside
(mixin_body) @class.inside
(enum_body) @class.inside

; 参数（around + inside）
(parameters) @parameter.around
(parameters
  "," @parameter.inside)

; 注释
(line_comment) @comment.around
(block_comment) @comment.around
(documentation_comment) @comment.around
