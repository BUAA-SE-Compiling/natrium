# 扩展 C0

这里列出了实现之后可以获得加分的扩展 C0 特性。

加分的单位尚未确定，目前的加分数量都是相对值。

## 注释

加分：5pt

```
COMMENT -> '//' regex(.*) '\n'
```

C0 的注释是从 `//` 开始到这行结束（遇到第一个 `\n`）为止的字符序列。注释不应当被词法分析输出。

## 字符字面量

加分：5pt

```
char_regular_char -> [^'\\\n\r]
CHAR_LITERAL -> '\'' (char_regular_char | escape_sequence)* '\''
literal_expr -> UINT_LITERAL | FLOAT_LITERAL | STRING_LITERAL | CHAR_LITERAL
```

字符字面量是由单引号 `'` 包裹的单个字符或转义序列。其中单个字符可以是 ASCII 中除了单引号 `'`、反斜线 `\\`、空白符 `\r` `\n` `\t` 以外的任何字符。转义序列可以是 `\'`、`\\`、`\n`、`\t`、`\r`，含义与 C 中的对应序列相同。

_字符字面量_ 的语义是被包裹的字符的 ASCII 编码无符号扩展到 64 位的整数值，类型是 `int`。

## 类型转换 & 浮点数类型

加分：25pt

### 类型转换

```
AS_KW -> 'as'
as_expr -> expr 'as' ty
expr -> .. | as_expr
```

显式类型转换通过 `as` 表达式实现。语言中没有隐式类型转换。

`表达式 as 类型` 表示将 `表达式` 的计算结果转换为 `类型` 所表示的类型的数据。`as` 表达式的左侧数据类型和右侧类型都不能是 `void`。

允许的类型转换包括：

- 类型 T 转换到它自己
- 浮点数 `double` 和整数 `int` 之间互相转换

### 浮点数类型

```
FLOAT_LITERAL -> digit+ '.' digit+ ([eE] digit+)?
```

浮点数类型 `double` 是遵循 IEEE 754 标准的 64 位浮点数（在其它语言中经常称作 `double`、`float64` 或 `f64`）。

浮点数和整数之间不能进行运算。浮点数之间进行四则运算的结果仍为浮点数。

## 布尔运算

TODO

## 作用域嵌套

加分：10pt

简而言之，在任何一个代码块中都可以声明变量。

要求：

- 每个代码块（`block_stmt`）都是一级作用域。
- 每级作用域内部的变量声明不能重复。
- 作用域内声明的变量可以覆盖上一级作用域中的变量。
- 每个作用域内定义的变量在作用域结束后即失效

比如，下面的函数中变量 `x`(1)、`counter`(2)、循环内的 `x`(3) 可以被访问的区域如竖线左侧所示：

```rust,ignore
    1  |  fn fib_iter(x: int) -> int {  // (1)
    |  |      let last_val: int = 1;
    |  |      let cur_val: int = 1;
  2 |  |      let counter: int = x - 2; // (2)
  | |  |      while counter > 0 {
3 |    |          let x: int = cur_val + last_val; // (3)
| |    |          last_val = cur_val;
| |    |          cur_val = x;
- |    |      }
  | |  |      return cur_val;
  - -  |  }
```

## 变量声明增强

加分：5pt

在每一级作用域中，你不仅可以在作用域顶端声明变量，也能在作用域中间声明变量。在作用域中间声明的变量同样遵循上一条的生命周期。

## `break` 和 `continue`

加分：10pt

```
BREAK_KW  -> 'break'
CONTINUE_KW -> 'continue'

break_stmt -> 'break' ';'

continue_stmt -> 'continue' ';'
```

- `break` 和 `continue` 必须在循环体内使用，在其他地方使用是编译错误。
- `break` 代表跳出循环体，控制转移到循环外的下一条语句。
- `continue` 代表跳过本次循环体的代码，控制转移到循环体的最后一条语句。

> 提示：进入循环之前记录一下跳转的目标位置

## 函数返回路径检查

加分：10pt

你需要对每一个函数的所有控制流进行检查，保证如果函数有返回值，那么所有可能的控制流都能导向 `return` 语句。比如，以下的函数不能通过编译：

```rust,ignore
fn foo(i: int) -> int {
    if i == 0 {
        return 1;
    } else {
        putint(0);
    }
    // 这个分支没有返回
}
```

这个也不行：

```rust,ignore
fn bar() -> int {
    let i: int;
    i = getint();
    while i > 0 {
        i = i - 1;
        if i <= 0 {
            return i;
        }
    }
    // 这个分支没有返回
}
```

这个可以，因为在到达函数结尾之前两个分支都返回了：

```rust,ignore
fn baz(i: int) -> int {
    if i == 0 {
        return 1;
    } else {
        return 0;
    }
    // 没有分支可以到达这里
}
```

> 提示：用基本块表示函数就能看得一清二楚了。

> UB: 我们不会考察对于无限循环的控制流检查。你可以选择报错，也可以选择无视。
