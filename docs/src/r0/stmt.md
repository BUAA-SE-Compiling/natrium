# 语句

```
stmt ->
      expr_stmt
    | decl_stmt
    | if_stmt
    | while_stmt
    | break_stmt
    | continue_stmt
    | return_stmt
    | block_stmt
    | empty_stmt
```

语句是

```

expr_stmt -> expr ';'

let_decl_stmt -> 'let' IDENT ':' ty ('=' expr)? ';'
const_decl_stmt -> 'const' IDENT ':' ty '=' expr ';'
decl_stmt -> let_decl_stmt | const_decl_stmt

if_stmt -> 'if' expr block_stmt ('else' 'if' expr block_stmt)* ('else' block_stmt)?

while_stmt -> 'while' expr block_stmt

break_stmt -> 'break' ';'

continue_stmt -> 'continue' ';'

return_stmt -> 'return' expr? ';'

block_stmt -> '{' stmt* '}'

empty_stmt -> ';'
```
