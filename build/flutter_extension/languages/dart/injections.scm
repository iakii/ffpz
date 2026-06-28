; ── Dart 内嵌语言注入查询 ──
; 用于识别 Dart 字符串中的嵌入式语言

; HTML 注入（用于 Dart 中的 HTML 字符串）
; 匹配 flutter_html 或 dart:html 场景
(argument_part
  (string_literal
    (string_content) @injection.content)
  (#match? @injection.content "<[a-zA-Z]")
  (#set! injection.language "html"))

; 正则表达式注入（在 RegExp 构造函数中的字符串）
(cascade_expression
  (identifier) @_id
  (#eq? @_id "hasMatch")
  (string_literal
    (string_content) @injection.content)
  (#set! injection.language "regex"))

; YAML 注入（用于 pubspec.yaml 内容的字符串）
; 注：这主要适用于 pubspec.yaml 文件，而不是 Dart 文件
