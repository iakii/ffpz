; ── Dart 可运行代码检测查询 ──
; 用于在编辑器中显示"运行"按钮

; 检测 main 函数（顶级入口点）
(function_declaration
  name: (identifier) @run
  (#eq? @run "main")
  body: (function_body)) @run

; 检测测试函数（test() 调用）
(argument_list
  (function_expression
    (parameters) @run
    (#any-of? @run "test" "group"))
  ) @run

; 检测以 test_ 或 _test.dart 文件中开头的函数
(function_declaration
  name: (identifier) @run
  (#match? @run "^test_")
  body: (function_body)) @run
