# 语句

```
stmt ->
      expr_stmt
    | decl_stmt
    | if_stmt
    | while_stmt
    | return_stmt
    | block_stmt
    | empty_stmt
```

语句是函数的最小组成部分。

## 表达式语句

```
expr_stmt -> expr ';'
```

表达式语句由 _表达式_ 后接分号组成。表达式如果有值，值将会被丢弃。

## 声明语句

```
let_decl_stmt -> 'let' IDENT ':' ty ('=' expr)? ';'
const_decl_stmt -> 'const' IDENT ':' ty '=' expr ';'
decl_stmt -> let_decl_stmt | const_decl_stmt
```

声明语句由 `let`（声明变量）或 `const`（声明常量）接 _标识符_、_类型_ 和可选的 _初始化表达式_ 组成。其中，常量声明语句必须有初始化表达式，而变量声明语句可以没有。

一个声明语句会在当前作用域中创建一个给定类型和标识符的变量或常量。声明语句有以下语义约束：

- 在同一作用域内，一个标识符只能由一个变量或常量使用。
- 变量或常量的类型不能为 `void`。
- 如果存在初始化表达式，其类型应当与变量声明时的类型相同。
- 常量只能被读取，不能被修改。

出现违反约束的声明语句是编译期错误。

> UB: 没有初始化的变量的值未定义。我们不规定对于使用未初始化变量的行为的处理方式，你可以选择忽略、提供默认值或者报错。

> UB: 我们不考虑局部变量和全局函数重名的情况。局部变量和全局变量重名的时候应当覆盖全局变量定义。

以下是一些可以通过编译的变量声明的例子：

```rust,ignore
let i: int;
let j: int = 1;
const k: double = 1.20;
```

以下是一些不能通过编译的变量声明的例子：

```rust,ignore
// 没有类型
let l = 1;
// 没有初始化
const m: int;
// 类型不匹配
let n: double = 3;
// 常量不能被修改
const p: double = 3.0;
p = 3.1415;
```

## 控制流语句

基础 C0 中有三种控制流语句，分别是 `if`、`while` 和 `return` 语句。

> 对于 `if` 和 `while` 的条件，如果求值结果是 `int` 类型，则所有非零值均视为 `true`。

### `if` 语句

```
if_stmt -> 'if' expr block_stmt ('else' (block_stmt | if_stmt))?
//              ^~~~ ^~~~~~~~~~         ^~~~~~~~~~~~~~~~~~~~~~
//              |     if_block           else_block
//              condition
```

`if` 语句代表一组可选执行的语句。

`if` 语句的执行流程是：

- 求 `condition` 的值
  - 如果值为 `true`，则执行 `if_block`
  - 否则，如果存在 `else_block`，执行 `else_block`
  - 否则，执行下一条语句

请注意，**if 语句的条件表达式可以没有括号**，且 **条件执行的语句都必须是代码块**。

以下是一些合法的 if 语句：

```rust,ignore
if x > 0 {
  x = x + 1;
}

if y < 0 {
  z = -1;
} else if y > 0 {
  z = 1;
} else {
  z = 0
}
```

以下是一些不合法的 if 语句：

```rust,ignore
// 必须是代码块
if x > 0 
  x = x + 1;
```

### `while` 语句

```
while_stmt -> 'while' expr block_stmt
//                    ^~~~ ^~~~~~~~~~while_block
//                     condition
```

while 语句代表一组可以重复执行的语句。

while 语句的执行流程是：

- 求值 `condition`
  - 如果为 `true`
    - 执行 `while_block`
    - 回到开头重新求值
  - 如果为 `false` 则执行之后的代码

### `return` 语句

```
return_stmt -> 'return' expr? ';'
```

使用 `return` 语句从一个函数中返回。return 语句可以携带一个表达式作为返回值。

return 语句有以下的语义约束：

- 如果函数声明的返回值是 `void`，return 语句不能携带返回值；否则，return 语句必须携带返回值
- 返回值表达式的类型必须与函数声明的返回值类型相同
- 当执行到返回值类型是 `void` 的函数的末尾时，应视作存在一个 return 语句进行返回

> UB: 在基础 C0 中不会出现部分分支没有返回值的情况，所以没有返回语句的分支的返回值是未定义的。在扩展 C0 中你必须检查每个分支都能够正常返回。

## 代码块

```
block_stmt -> '{' stmt* '}'
```

一个代码块可以包含一条或多条语句。执行代码块的效果是顺序执行这些语句。

在基础 c0 中，一个代码块中的声明语句只能在其他类型的语句之前出现。

在扩展 c0（作用域嵌套）中，一个代码块是其所在作用域的子作用域。在扩展 c0（变量声明增强）中，一个代码块的任何地方均可声明变量。

## 空语句

```
empty_stmt -> ';'
```

空语句没有任何作用，只是一个分号而已。
