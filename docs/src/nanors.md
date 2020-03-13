# NanoRs 语法说明

NanoRs 是一个用于编译原理课程的微型语言。为了减少编译器实现的压力（减少前瞻和/或回溯），语言的语法比较接近 Rust。

### 标识符

```
// # Identifier
identifier -> [_a-zA-Z] [_0-9a-zA-Z]
```

### 表达式

```
block -> "{" statement* "}"

binary_operator ->
    | "+" | "-" | "*" | "/" 
    | "=" | ">" | "<" | ">=" | "<=" | "==" | "!="
    | "&" | "|" | "&&" | "||" | "^" 
    | "<<" | ">>" 

preceding_unary_operator ->
    | "&" | "*" | "!" | "~"

literal_expr -> literal
preceding_unary_expr -> preceding_unary_operator expr
binary_expr -> expr (binary_operator expr)*
call_params -> expr ("," expr)* ","?
method_call_expr -> expr "(" call_params? ")"
group_expr -> "(" expr ")"
type_conversion_expr -> expr "as" identifier

expr -> 
    | literal_expr
    | preceding_unary_expr
    | binary_expr
    | group_expr
    | type_conversion_expr
```

NanoRs 中的表达式有五种，其结合性按照运算符优先级计算。运算符优先级从高到低如下：

| 运算符                      | 结合性 |
| --------------------------- | ------ |
| 函数调用                    | -      |
| 前置单目 `&` `*` `!` `~`    | 右到左 |
| `as`                        | 左到右 |
| `*` `/`                     | 左到右 |
| `+` `-`                     | 左到右 |
| `<<` `>>`                   | 左到右 |
| `&`                         | 左到右 |
| `^`                         | 左到右 |
| <code>&#124;</code>         | 左到右 |
| `==` `<` `>` `<=` `>=` `!=` | 左到右 |
| `&&`                        | 左到右 |
| <code>&#124;&#124;</code>   | 左到右 |
| `=`                         | 右到左 |



### 语句

```
let_stmt -> "let" identifier ":" type ";"
if_stmt -> "if" expr block ("else" block)?
while_stmt -> "while" expr block

var_declaration -> identifier ":" type
fn_param_list -> var_declaration ("," var_declaration)* ","?
function -> "fn" identifier "(" fn_param_list ")" block
```
