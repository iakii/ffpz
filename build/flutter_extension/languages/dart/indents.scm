; ── Dart 自动缩进规则 ──

; 在花括号块内缩进
(block) @indent

; switch/case 块缩进
(switch_case
  body: (_) @indent)

; 列表、映射、记录字面量
(list_literal) @indent
(set_literal) @indent
(map_literal) @indent
(record_literal) @indent

; 参数列表缩进
(parameters) @indent
(argument_list) @indent

; 函数体缩进
(function_body) @indent

; 修复缩进：在关闭括号后取消缩进
"]" @outdent
"}" @outdent
")" @outdent
