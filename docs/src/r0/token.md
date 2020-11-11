# 单词 (Token)

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
BREAK_KW  -> 'break'
CONTINUE_KW -> 'continue'
```

# 字面量

```
digit -> [0-9]
UINT_LITERAL -> digit+
FLOAT_LITERAL -> digit+ '.' digit+ ([eE] digit+)?

escape_sequence -> '\' [\\"'nrt]
string_regular_char -> [^"\\]
STRING_LITERAL -> '"' (string_regular_char | escape_sequence)* '"'

char_regular_char -> [^'\\]
CHAR_LITERAL -> '\'' (char_regular_char | escape_sequence)* '\''
```

## 标识符

```
IDENT -> [_a-zA-Z] [_a-zA-Z0-9]*
```

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

```
COMMENT -> '//' regex(.*) '\n'
```
