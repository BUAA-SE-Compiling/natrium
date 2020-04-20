# 设计笔记与讨论

> 这里存放着设计 2020 软院编译原理所使用的语言的时候所考虑的一些东西。

## 语法

> 邵老师：最好不要自创一门新的语言或者改编自小众语言。
> 
> Rynco：是否这么去做还没有确定。我个人是倾向于创建一门新的语言的
> 

考点：递归下降分析

> 按照 hambaka 的意思，c0 的语法可以进一步简化，降低实现难度。

Hambaka 建议的语法修改包括：

- 去除隐式类型转换，所有类型转换必须显式声明
- 只保留 while 和/或 for 作为唯一/二可用的循环
- 去除 switch 语句

Rynco 正在考虑的语法修改包括：

- 类型后置
- 规范 bool 类型

> Rynco: 
> 
> 我计划的是一个长得有点 Rust（只借鉴了关键字）的 C，不知道其他人怎么看
> 
> 我希望这么做的原因是：如果使用类型后置的话，就可以把类型关键字解析完全放在语义解析里面做了。从而不需要在 parse 的时候维护一个符号表来确定哪些标识符是类型、哪些是变量，或者在解析到类型转换时回溯了（见 [附录1][ref1] ）。
 
[ref1]: #附录1：C 风格的语法在解析时的回溯问题与解决方案
 
```rust
let global_var: double = -123456.789e10;

fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() -> void {
    let a: int = 3;
    let b: int = 4;
    let c: double = add(a, b) as double;
    if global_var > 0 {
        print("Hello");
    } else {
        print(c);
    }
}
```

> 自动类型推断应该不会有了……吧？有人要当进阶内容做我不反对。

## 类型系统

考点：类型转换

虚拟机的设计默认使用 64 位整数和浮点数。

会有整数和浮点数（没法用同一种类型假装能转换的）

虚拟机的设计支持数组、结构体和堆内存分配，这三者可以作为进阶内容选做。


## 附录

### 附录1：C 风格的语法在解析时的回溯问题与解决方案

考虑以下关于赋值语句的语法规则（不规定 `int`、`double` 等类型为关键字）：

```
# 标识符
ident -> "int" | "x"
# 运算符
op -> "+" | "-"

# 类型，在词法分析时被分析为标识符
ty -> ident
# 表达式
expr -> "0" | "1" | ident | expr op expr

# 变量声明语句
decl_stmt -> ty ident "=" expr ";"
# 表达式语句
expr_stmt -> expr ";"

# 语句
stmt -> decl_stmt | expr_stmt
```

显然，FIRST(`decl_stmt`) ∩ FIRST(`expr_stmt`) == { `ident` }。鉴于大部分同学在实现的时候都会考虑采用递归下降分析法（这门课的大作业里不会真的有人去手写/生成 LL/LR 吧？），这个 FIRST 集的重合会造成大量的回溯。

类似的还有显式类型转换：

```
ident -> "int" | "x"
ty -> ident

# 括号表达式
paren_expr -> "(" expr ")"
# C 风格的显式类型转换
cast_expr -> "(" ty ")" expr

expr -> ident | cast_expr | paren_expr
```

如果没有在语法分析的时候就建立符号表，在分析代码 `(int)(x)` 时甚至在读完 `(int)` 这三个 token 之后都不能确定解析的表达式到底是类型转换还是括号。这会给分析器的实现造成较大的障碍。

因此，我建议在设计语法的时候就考虑这类问题，避免大量的 FIRST 集重合现象，降低递归下降语法分析器的实现难度。具体方案如下：

> 在以下的语法规则中，`ty` 代表类型，`ident` 代表标识符，`expr` 代表表达式，`block` 代表语法块。
> 

一，将变量和函数声明中的类型后置，使用关键字开始此类语句，避免与 `expr` 的 FIRST 集重叠。此语法与多种现代语言（如 TypeScript、Kotlin、Go、Rust）相类似。

```
# 修改前:

decl_stmt -> "const"? ty ident ("=" expr)? ";"
# int myVariable = 123 + 456;

function_param -> ty ident ("," ty ident)*
function -> ty ident "(" function_param? ")" block
# int add(int a, int b) { ... }

# 修改后:

decl_stmt -> ("let" | "const") ident ":" ty ("=" expr)? ";"
# let myVariable: int = 123 + 456;

function_param -> ident ":" ty ("," ident ":" ty)*
function -> "fn" ident "(" function_param? ")" "->" ty block
# fn add(a: int, b: int) -> int { ... }
```

二，将显式类型转换的语法从括号变为使用 `as` 做运算符的二元表达式。此语法与多种现代语言（如 TypeScript、Kotlin、Rust、C#）相类似。

```
# 修改前：

cast_expr -> "(" ty ")" expr
# (int)42.0
# (double)(a + b)

# 修改后：

cast_expr -> expr "as" ty
# 42.0 as int
# (a + b) as double
```
