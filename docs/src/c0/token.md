# 单词 (Token)

单词是词法分析的结果。

## 关键字

```
FN_KW     -> 'fn'
LET_KW    -> 'let'
CONST_KW  -> 'const'
AS_KW     -> 'as'
WHILE_KW  -> 'while'
IF_KW     -> 'if'
ELSE_KW   -> 'else'
RETURN_KW -> 'return'

// 这两个是扩展 c0 的
BREAK_KW  -> 'break'
CONTINUE_KW -> 'continue'
```

c0 有 8 个关键字。扩展 c0 增加了 2 个关键字。

# 字面量

```
digit -> [0-9]
UINT_LITERAL -> digit+
FLOAT_LITERAL -> digit+ '.' digit+ ([eE] digit+)?

escape_sequence -> '\' [\\"'nrt]
string_regular_char -> [^"\\]
STRING_LITERAL -> '"' (string_regular_char | escape_sequence)* '"'

// 扩展 c0 的字符常量
char_regular_char -> [^'\\]
CHAR_LITERAL -> '\'' (char_regular_char | escape_sequence)* '\''
```

c0 有三种字面量，分别是 _无符号整数_、_浮点数_ 和 _字符串常量_。

## 标识符

```
IDENT -> [_a-zA-Z] [_a-zA-Z0-9]*
```

c0 的标识符由下划线或字母开头，后面可以接零或多个下划线、字母或数字。标识符不能和关键字重复。

## 运算符

```
PLUS      -> '+'
MINUS     -> '-'
MUL       -> '*'
DIV       -> '/'
ASSIGN    -> '='
EQ        -> '=='
NEQ       -> '!='
LT        -> '<'
GT        -> '>'
LE        -> '<='
GE        -> '>='
L_PAREN   -> '('
R_PAREN   -> ')'
L_BRACE   -> '{'
R_BRACE   -> '}'
ARROW     -> '->'
COMMA     -> ','
COLON     -> ':'
SEMICOLON -> ';'
```

## 注释

注释是扩展 c0 内容，见 [扩展 c0](extended-c0.md#注释)

```
COMMENT -> '//' regex(.*) '\n'
```
