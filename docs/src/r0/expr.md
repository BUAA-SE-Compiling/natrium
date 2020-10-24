# 表达式与语句

## 表达式

表达式是代码中运算的最小单位。在语法解析的时候，一个表达式会被展开成一棵树，称作表达式树。

### 运算符

r0 一共有 15 种运算符，全部为双目运算符。它们分别是：

- 赋值运算符 `=`
- 算数运算符 `+` `-` `*` `/`
- 比较运算符 `>` `<` `>=` `<=` `==` `!=`
- 类型转换运算符 `as`
- 布尔运算符 `&&` `||` `!` （选做；待定）
<!-- - 按位运算符 `&` `|` `^` `>>` `<<` `~` （选做；待定） -->

<!-- 其中， `*` `&` 还可以作为前置单目运算符作为解引用、取引用使用（选做；待定）。 -->

除 `as` 以外，其他类型的运算符的两侧必须是相同类型的数据。各运算符含义如下：

| 运算符 | 含义                       | 参数类型   | 结果类型   | 结合性 |
| ------ | -------------------------- | ---------- | ---------- | ------ |
| `=`    | 将右侧值赋给左侧           | 左值, 右值 | void       | 右到左 |
| `+`    | 将左右两侧相加             | 数值       | 与参数相同 | 左到右 |
| `-`    | 将左右两侧相减             | 数值       | 与参数相同 | 左到右 |
| `*`    | 将左右两侧相乘             | 数值       | 与参数相同 | 左到右 |
| `/`    | 将左右两侧相除             | 数值       | 与参数相同 | 左到右 |
| `>`    | 如果左侧大于右侧则为真     | 数值       | bool       | 左到右 |
| `<`    | 如果左侧小于右侧则为真     | 数值       | bool       | 左到右 |
| `>=`   | 如果左侧大于等于右侧则为真 | 数值       | bool       | 左到右 |
| `<=`   | 如果左侧小于等于右侧则为真 | 数值       | bool       | 左到右 |
| `==`   | 如果左侧等于右侧则为真     | 数值       | bool       | 左到右 |
| `!=`   | 如果左侧不等于右侧则为真   | 数值       | bool       | 左到右 |
| `as`   | 将左侧值转换到右侧类型     | 值, 类型   | 右侧类型   | 左到右 |

<!-- | `&&`   | 如果左右都为真则为真       | bool       | bool       | 左到右 |
| `||`   | 如果左右有一个为真则为真   | bool       | bool       | 左到右 | -->
<!-- &#124; -->

### 函数调用

```
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

## 语句

```
block -> "{" stmt* "}"

ident_list -> ident ("," ident)*
let_stmt -> "let" ident_list ":" type ";"
if_stmt -> "if" expr block ("else" block)?
while_stmt -> "while" expr block
return_stmt -> "return" expr ";"

stmt ->
    | let_stmt
    | if_stmt
    | expr_stmt
    | return_stmt

var_declaration -> identifier ":" type
fn_param_list -> var_declaration ("," var_declaration)* ","?
function -> "fn" identifier "(" fn_param_list ")" block
```
