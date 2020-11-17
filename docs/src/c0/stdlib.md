# 标准库

由于 c0 语言本身比较简单，为了实现输入输出的功能，我们规定了 8 个不需要声明就可以调用的函数，它们的分别是：

```rust,ignore
/// 读入一个有符号整数
fn getint() -> int;

/// 读入一个浮点数
fn getdouble() -> double;

/// 读入一个字符
fn getchar() -> int;

/// 输出一个整数
fn putint(int) -> void;

/// 输出一个浮点数
fn putdouble(double) -> void;

/// 输出一个字符
fn putchar(int) -> void;

/// 输出整数代表的全局常量字符串
fn putstr(int) -> void;

/// 输出一个换行
fn putln() -> void;
```

在实现时，这些函数既可以编译成使用虚拟机中的 `callname` 指令调用，也可以编译成相应的虚拟机指令（`scan.i`, `print.i` 等），在虚拟机实现上两者是等价的。
