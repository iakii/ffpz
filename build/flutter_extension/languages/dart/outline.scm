; ── Dart 代码大纲查询 ──
; 用于生成侧边栏的文件结构概览

; 类定义
(class_declaration
  name: (identifier) @name
  body: (class_body) @item) @context

; Mixin 定义
(mixin_declaration
  name: (identifier) @name
  body: (mixin_body) @item) @context

; Enum 定义
(enum_declaration
  name: (identifier) @name
  body: (enum_body) @item) @context

; Extension 定义
(extension_declaration
  name: (identifier) @name
  body: (extension_body) @item) @context

; 顶级函数
(function_declaration
  name: (identifier) @name
  body: (function_body) @item) @context

; 方法定义
(method_declaration
  name: (identifier) @name
  body: (block) @item) @context

; Getter
(getter_signature
  name: (identifier) @name
  body: (function_body) @item) @context

; Setter
(setter_signature
  name: (identifier) @name
  body: (function_body) @item) @context

; 构造函数
(constructor_declaration
  name: (identifier) @name
  body: (function_body) @item) @context

; 顶级变量
(initialized_variable_declaration
  name: (identifier) @name)
(final_declaration
  name: (identifier) @name)

; 顶级 typedef
(typedef_definition
  name: (identifier) @name) @context
