# 设计笔记与讨论

> 这里存放着设计 2020 软院编译原理所使用的语言的时候所考虑的一些东西。
> 

> Rynco：我认为，实验的目标应当是让学生尽可能多的了解一个真实的编译器是如何运行的。因此，我们需要尽可能消减不必要的内容，比如复杂的指令集、寄存器分配、过于繁琐的语法等等。剩下来的应该是一个只包含核心内容的现代语言。

## 语法

> 邵老师：最好不要自创一门新的语言或者改编自小众语言。
> 
> Rynco：我个人是倾向于创建一门新的语言的。

考点：词法、语法分析

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
> 我计划的是一个长得有点 Rust（只借鉴了关键字和部分语法，因为解析器写起来容易）的 C，不知道其他人怎么看
> 
> 我希望这么做的原因是：如果使用类型后置的话，就可以把类型关键字解析完全放在语义解析里面做了。从而不需要在 parse 的时候维护一个符号表来确定哪些标识符是类型、哪些是变量，或者在解析到类型转换时回溯了（见 [附录1][ref1] ）。
 
[ref1]: #附录1：C 风格的语法在解析时的回溯问题与解决方案
 
```rust,ignore
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

> Rynco: 自动类型推断应该不会有了……吧？有人要当进阶内容做我不反对。
> 

### 暂定的语法内容

> Rynco: 这里决定的内容会在实现示例编译器的时候再次确认，保证实现编译器的工作量不会过大。如果实现示例编译器的时候发现什么地方难度偏大的话还会再砍。

#### 字面量

字面量包括以下内容：

- 整数
- 浮点数
- 字符
- 字符串
- 布尔值

#### 运算

算术运算：

- 相反数
- 加
- 减
- 乘
- 除

比较运算：

- 大于
- 小于
- 大于等于
- 小于等于
- 等于
- 不等于

赋值运算：

- 赋值

进阶版本可以支持以下运算（虚拟机已支持）：

- 布尔非
- 布尔与
- 布尔或
- 按位与
- 按位或
- 按位异或
- 左移
- 算术右移
- 逻辑右移

#### 变量声明与赋值

变量声明使用 `let` 或 `const` 关键字声明，语法见 附录1。

赋值表达式使用等号 `=` 作为运算符，运算结果类型为空值（`void`）。

> 比如 `(a = b) == c` 是合法的表达式，但是类型检查会出错。赋值作为表达式而不是语句的原因是赋值语句和表达式的 FIRST 集会相交。

要求实现变量作用域。

#### 条件表达式和循环

一种条件表达式：`if-elseif-else` 表达式。

一种循环：`while` 循环。

#### 函数

函数使用 `fn` 关键字声明，语法见 附录1。

不要求实现前向引用。


## 类型系统

考点：类型转换

虚拟机的设计默认使用 64 位整数和浮点数。

会有整数和浮点数（没法用同一种类型假装能转换的）

虚拟机的设计支持数组、结构体和堆内存分配，这三者可以作为进阶内容选做。

### 暂定的类型系统

必做

- `int` (`i64`)
- `double` (`f64`)
- `void` (`unit` / `()`)

进阶（待砍刀）

- `bool`
    (存储上等同于 `u8`，`false == 0u8`, `true == 255u8`)
- `u8`/`u16`/`u32`/`u64`
- `i8`/`i16`/`i32`/`i64`
- struct
- array (`[T; N]`)
- pointer (`&T`)
- 自动类型推断（省去 `let variable` 后面的类型）

## 虚拟机

考点：代码生成

编译目标是 r0vm 虚拟机，是栈式虚拟机。编译到 JVM / Dotnet CLR / x86 等目标如果想做的话可以选做，需要提前跟助教声明。

虚拟机设计已经基本确定，见相关文档。

## 附录

### 附录1：C 风格的语法在解析时的回溯问题与解决方案 

考虑以下 C 风格的变量声明语句的语法规则（`int`、`double` 等类型名称不是关键字）：

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

如果没有在语法分析的时候就建立符号表，在分析代码 `(int)(x)` 时甚至在读完 `(int)` 这三个 token 之后都不能确定解析的表达式到底是类型转换还是括号。为了解决这个问题，要么需要预读不确定数量的 token （在类型声明可能大于 1 个 token 时），要么遇到括号就要准备回溯，两者在代码上实现难度都偏大。

因此，我建议在设计语法的时候就考虑这类问题，避免大量的 FIRST 集重合现象，降低递归下降语法分析器的实现难度。具体方案如下：

> 在以下的语法规则中，`ty` 代表类型，`ident` 代表标识符，`expr` 代表表达式，`block` 代表语法块。
> 

一，将变量和函数声明中的类型后置，使用关键字开始此类语句，避免与 `expr` 的 FIRST 集重叠。此语法与多种现代语言（如 TypeScript、Kotlin、Go、Rust）相类似。

> 九个六：`const` 可以考虑砍掉（待定）

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


### 附录2：九个六先生的例程

```rust,ignore
let a1, a2, a3, a4, a5: int;

fn me(x: int) -> int {
 return x;
}

fn add(x: double, y: double) -> int {
    let xx: int = x as int;
    let yy: int = y as int;
    return xx + yy
}

fn give(idx: int) -> int {
    if idx == 1 {
        return a1;
    }
    else if idx == 2 {
        return a2;
    }
    else if idx == 3 {
        return a3;
    }
    else if idx == 4 {
        return a4;
    }
    else if idx == 5 {
        return a5;
    }
    else {
        return 114514.0 as int
    }
}

fn set(idx: int, val: int) -> void {
    if idx == 1 {
        a1 = val;
    }
    else if idx == 2 {
        a2 = val;
    }
    else if idx == 3 {
        a3 = val;
    }
    else if idx == 4 {
        a4 = val;
    }
    else if idx == 5 {
        a5 = val;
    }
}

fn main() -> void {
    let a, b, c, t: int;
    let five: int = 5;
    a = getint();
    b = getint();
    set(1, a);
    set(2, b);
    t = 3;
    while t <= five {
        c = add(a as double, b as double);
        b = a;
        a = c;
        set(t, c);
        t = t + 1;
    }
    print(give(me(five)));
```
