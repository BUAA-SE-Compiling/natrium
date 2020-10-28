# r0 虚拟机标准

本次大作业的编译目标是 r0 虚拟机 (r0vm) 的汇编 (s0)。其设计参考了 JVM、DotNet CLR 和上学期的 c0 虚拟机。

## 虚拟机简介

r0vm 是一个 [栈式虚拟机][stack_machine] —— 简单来说就是，它的寄存器是一个栈。除了少数内存访问指令以外，r0vm 的大部分指令都只操作位于栈顶的数据。堆栈式计算机的指令与 [逆波兰表示法（后缀表示法][reverse_polish_notation] 表示的表达式（或者说后序遍历的表达式树）有简单的对应关系。

r0vm 有 64 位有符号整数、无符号整数、浮点数三种数据类型。详见 [数据类型](#数据类型) 节。

r0vm 使用 64 位的地址空间，详见 [地址空间](#地址空间) 节。

r0vm 使用自制的指令集，共有 50+ 个指令，详见 [指令集](#指令集) 节。

[stack_machine]: https://en.wikipedia.org/wiki/Stack_machine
[reverse_polish_notation]: https://en.wikipedia.org/wiki/Reverse_Polish_notation

## 内存空间

r0vm 的内存空间以 8 位（1 字节）为单位寻址。8、16、32、64 位的数据类型分别以其大小为单位在内存中对齐。当读取或写入操作未对齐时，会产生 `UnalignedAccess` 错误。

r0vm 的栈空间以 8 字节为一个 slot，压栈、弹栈以及各种运算操作均以 slot 为单位进行。默认情况下，栈的大小是 1 MiB (1048576 字节)，即 131072 个 slot。栈空时弹栈和栈满时压栈分别会产生 `StackUnderflow` 和 `StackOverflow` 错误。

> 以 r0vm 为编译目标的话没有必要了解地址空间的结构，写在这里只是作为笔记使用。除非……你想在代码里判断一个指针是指向堆还是栈的（没必要，真的没必要）

r0vm 的全局变量、堆和栈在同一个线性地址空间内。地址的宽度是 64 bit。其中

- 全局变量从地址空间最底端 `0x0000_0000_0000_0000` 向上延伸至（不含） `0x0000_0001_0000_0000`
- 栈从地址空间上端 `0xffff_ffff_0000_0000` 向上延伸，且不超过规定的栈大小内
- 堆从地址 `0x0000_00001_0000_0000` 向上延伸。

地址空间示意图：

```
|--------------| <- 0xffff_ffff_ffff_ffff
| //////////// |
|--------------|
| 栈   ^       |
|      |       |
|--------------| <- 0xffff_ffff_0000_0000
| //////////// |
| //////////// |
|--------------| 
|       ^      |
|       |      |
| 堆           |
|--------------| <- 0x0000_0001_0000_0000
| //////////// |
|--------------|
| 全局变量     |   
|--------------| <- 0x0000_0000_0000_0000
```

## 数据类型

r0vm 在运算中支持三种基本数据类型，分别是 64 位无符号整数 `u64`、64 位有符号整数 `i64`、64 位浮点数 `f64`。长度更短的整数可以使用 `u64` 和 `i64` 模拟。

`u64` 和 `i64` 都是 64 位整数，使用[二进制补码][2s_complement]形式表示。两种类型在多数整数运算中不做区分，仅在 `cmp.T`（比较指令，见下）等两种运算结果有差别的地方有所区分。在运算溢出时，两种类型均采用环绕 (wrap-around) 方式处理结果。`u64` 同时也可以表示虚拟机中的内存地址。

`f64` 是符合 [IEEE 754][ieee754] 规定的[双精度浮点数][double]。

[2s_complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[ieee754]: https://en.wikipedia.org/wiki/IEEE_754
[double]: https://en.wikipedia.org/wiki/Double-precision_floating-point_format

## 二进制格式

> TODO: 格式还没确定

s0 是 r0vm 所使用的汇编文件格式，其作用和内容类似 Java 的 `.class` 文件或者 DotNet 的 `.dll` 文件。

下面的结构体表示了 s0 的二进制文件结构。其中，`uXX` 表示 XX 位无符号整数 。

```
/// 整个 S0 二进制文件
struct S0 {
    /// 魔数
    magic: u32 = 0x72303b3e,
    /// 版本号，定为 1
    version: u32 = 0x00000001,
    /// 标志位，内容未确定
    flags: u32,
    /// 全局变量表
    globals: Array<GlobalDef>,
    /// 函数列表
    functions: Array<FunctionDef>,
}

/// 数组
struct Array<T> {
    count: u32,
    items: T[],
}

/// 单个全局变量
struct GlobalDef {
    /// 是否为常量？非零值视为真（待确定）
    is_const: u8,
    /// 值
    value: Array<u8>,
}

/// 函数
struct FunctionDef {
    /// 函数名称在全局变量中的位置
    name: u16,
    /// 返回值占据的 slot 数
    return_slots: u16,
    /// 参数占据的 slot 数
    param_slots: u16,
    /// 局部变量占据的 slot 数
    loc_slots: u16,
    /// 函数体
    body: Array<Instruction>,
}

/// 指令
union Instruction {
    /// 无参数的指令
    variant NoParam {
        opcode: u8
    },
    /// 有 4 字节参数的指令
    variant u32Param {
        opcode: u8,
        param: u32,
    }
    /// 有 8 字节参数的指令
    variant u64Param {
        opcode: u8,
        param: u64
    }
}
```

## 栈帧结构

> 这里描述的是 **这个** r0vm 实现中使用的栈帧结构。其他实现可能与本实现具有不同的栈帧结构，和/或使用不同的方式传递参数、储存局部变量等。

```
| ...          |
|              | <- 栈顶 %sp
| 表达式栈 ... |
| 表达式栈     |
| 局部变量 ... |
| 局部变量     |
| 虚拟机参数...| 
| 虚拟机参数   | <- 被调用者栈底 %bp
| 调用参数 ... | 
| 调用参数     | 
|--------------| ===
| 返回值       | v 
| 中间结果     | 调用者栈
| ...          | ^
|--------------| ===
```

其中，调用参数和返回值由调用者压栈，调用参数在函数返回后由被调用者清理。

### 虚拟机参数

虚拟机会在调用参数和局部变量之间插入一系列的虚拟机参数以辅助虚拟机运行，目前本虚拟机存储的参数格式为（从栈顶到栈底）：

```
| ...           |
| 局部变量      | 
|---------------|
| 调用者函数 ID | 
| 调用者 %ip    | 
| 调用者 %bp    | 
|---------------|
| 参数          | 
| ...           |
```

### 函数调用时栈帧变化示例

假设现有一个函数 `test`，有 1 slot 的返回值、2 slot 的参数和 2 slot 的局部变量。

```rust
/// 可以看成是这样的一个函数
fn test(a: int, b: int) -> int {
    let c: int = ...;
    let d: int = ...;
    ...
    return ...;
}
```

现在，它被编号为 1 的函数 `main` 调用，则执行 `call` 指令前的栈如下（不规定参数压栈顺序）：

```
| -          |
|------------| <- 栈顶
| b          | ↑
| a          | 参数
| _ret       | 返回值
| ...        | ...表达式栈
```

在执行 `call` 指令后，栈如下：

```
| -          | <- 栈顶（表达式栈）
| d          | ↑
| c          | 局部变量
|------------|
| 1          | ↑
| %ip        | |
| %bp        | 虚拟机数据
|------------|
| b          | ↑
| a          | 参数
| _ret       | 返回值
| ...        |
```

在函数调用返回后，栈如下：

```
| -          | 
| // d       |  
| // c       |
| // 1       | 
| // %ip     | 
| // %bp     |  ↑
| // b       |  |
| // a       | 以上内容被弹栈

|------------| <- 栈顶
| _ret       | 返回值
| ...        |
```

## 程序入口

r0vm 总是会最先运行函数列表里编号为 0 的（也就是整个列表中第一个）函数，按照惯例这个函数的名称为 `_start`。`_start` 函数没有任何参数，也不返回任何值。这两项的参数会被忽略。

一般来说，程序会在 `_start` 中设置全局变量的值，以及进行其他的准备工作。在准备工作完成之后，`_start` 函数应当调用 `main` 函数开始正式的程序运行。如果需要，`_start` 函数也可以在 `main` 函数返回之后进行清理工作。`_start` 函数不需要返回。

一个示例的 `_start` 函数如下：

```
fn _start 0 0 -> 0 {
    // global(1) = 1 + 1;
    globa 1
    push 1
    push 1
    add.i
    store.64
    // main();
    call 4
    // cleanup: global(1) = 0;
    globa 1
    push 0
    store.64
    // no return
}
```

## 指令集

r0vm 的指令使用 8 位无符号整数标识，后面跟随可变长度的操作数。

下表展示了 r0vm 的所有指令。其中弹栈和压栈的格式为：`栈变化范围[:变量]`，数字按照栈底到栈顶编号。

| 指令 | 指令名       | 操作数   | 弹栈          | 压栈         | 介绍                                    |
| ---- | ------------ | -------- | ------------- | ------------ | --------------------------------------- |
| 0x00 | `nop`        | -        | -             | -            | 空指令                                  |
| 0x01 | `push`       | num:u64  | -             | 1:num        | 将 num 压栈                             |
| 0x02 | `pop`        | -        | 1             |              | 弹栈 1 个 slot                          |
| 0x03 | `popn`       | num:u32  | 1-num         | -            | 弹栈 num 个 slot                        |
| 0x04 | `dup`        | -        | 1:num         | 1:num, 2:num | 复制栈顶 slot                           |
| 0x08 | `loca`       | off:u32  | -             | 1:addr       | 加载 off 偏移量处局部变量的地址         |
| 0x08 | `arga`       | off:u32  | -             | 1:addr       | 加载 off 偏移量处参数的地址         |
| 0x09 | `globa`      | n:u32    | -             | 1:addr       | 加载第 n 个全局变量/常量的地址          |
| 0x10 | `load.8`     | -        | 1:addr        | 1:val        | 从 addr 加载 8 位 value 压栈            |
| 0x11 | `load.16`    | -        | 1:addr        | 1:val        | 从 addr 加载 16 位 value 压栈           |
| 0x12 | `load.32`    | -        | 1:addr        | 1:val        | 从 addr 加载 32 位 value 压栈           |
| 0x13 | `load.64`    | -        | 1:addr        | 1:val        | 从 addr 加载 64 位 value 压栈           |
| 0x14 | `store.8`    | -        | 1:val, 2:addr | -            | 把 val 截断到 8 位存入 addr             |
| 0x15 | `store.16`   | -        | 1:val, 2:addr | -            | 把 val 截断到 16 位存入 addr            |
| 0x16 | `store.32`   | -        | 1:val, 2:addr | -            | 把 val 截断到 32 位存入 addr            |
| 0x17 | `store.64`   | -        | 1:val, 2:addr | -            | 把 val 存入 addr                        |
| 0x18 | `alloc`      | -        | 1:size        | 1:addr       | 在堆上分配 size 字节的内存              |
| 0x19 | `free`       | -        | 1:addr        | -            | 释放 addr 指向的内存块                  |
| 0x1a | `stackalloc` | size:u32 | -             | -            | 在当前栈顶分配 size 个 slot，初始化为 0 |
| 0x20 | `add.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs + rhs，参数为整数        |
| 0x21 | `sub.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs - rhs，参数为整数        |
| 0x22 | `mul.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs * rhs，参数为整数        |
| 0x23 | `div.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs / rhs，参数为有符号整数  |
| 0x24 | `add.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs + rhs，参数为浮点数      |
| 0x25 | `sub.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs - rhs，参数为浮点数      |
| 0x26 | `mul.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs * rhs，参数为浮点数      |
| 0x27 | `div.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs / rhs，参数为浮点数      |
| 0x28 | `div.u`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs / rhs，参数为无符号整数  |
| 0x29 | `shl`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs << rhs                   |
| 0x2a | `shr`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs >> rhs （算术右移）      |
| 0x2b | `and`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs & rhs                    |
| 0x2c | `or`         | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs &#124; rhs               |
| 0x2d | `xor`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs ^ rhs                    |
| 0x2d | `not`        | -        | 1:lhs         | 1:res        | 计算 res = !lhs                         |
| 0x2e | `inv`        | -        | 1:lhs         | 1:res        | 计算 res = ~lhs（按位反转）             |
| 0x30 | `cmp.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较有符号整数 lhs 和 rhs 大小          |
| 0x31 | `cmp.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较浮点数 lhs 和 rhs 大小              |
| 0x32 | `cmp.u`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较无符号整数 lhs 和 rhs 大小          |
| 0x34 | `neg.i`      | -        | 1:lhs         | 1:res        | 对 lhs 取反                             |
| 0x35 | `neg.f`      | -        | 1:lhs         | 1:res        | 对 lhs 取反                             |
| 0x36 | `itof`       | -        | 1:lhs         | 1:res        | 把 lhs 从整数转换成浮点数               |
| 0x37 | `ftoi`       | -        | 1:lhs         | 1:res        | 把 lhs 从浮点数转换成整数               |
| 0x38 | `shrl`       | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs >>> rhs （逻辑右移）     |
| 0x40 | `bra`        | -        | 1:addr        |              | 无条件跳转到地址 `addr`                 |
| 0x41 | `br`         | off:i32  |               |              | 无条件跳转偏移 `off`                    |
| 0x42 | `bz`         | off:i32  |               | 1:test       | 如果 `test` 是 0 则跳转偏移 `off`       |
| 0x43 | `bnz`        | off:i32  |               | 1:test       | 如果 `test` 非 0 则跳转偏移 `off`       |
| 0x44 | `bl`         | off:i32  |               | 1:test       | 如果 `test` 小于 0 则跳转偏移 `off`     |
| 0x45 | `bg`         | off:i32  |               | 1:test       | 如果 `test` 大于 0 则跳转偏移 `off`     |
| 0x46 | `blz`        | off:i32  |               | 1:test       | 如果 `test` 小于等于 0 则跳转偏移 `off` |
| 0x47 | `bgz`        | off:i32  |               | 1:test       | 如果 `test` 大于等于 0 则跳转偏移 `off` |
| 0x48 | `call`       | id:u32   | 见栈帧介绍    | -            | 调用编号为 id 的函数                    |
| 0x49 | `ret`        | -        | -             | 见栈帧介绍   | 从当前函数返回                          |
| 0x50 | `scan.i`     | -        | -             | 1:n          | 从标准输入读入一个整数 n                |
| 0x51 | `scan.c`     | -        | -             | 1:c          | 从标准输入读入一个字符 c                |
| 0x52 | `scan.f`     | -        | -             | 1:f          | 从标准输入读入一个浮点数 f              |
| 0x54 | `print.i`    | -        | 1:x           | -            | 向标准输出写入一个有符号整数 x          |
| 0x55 | `print.c`    | -        | 1:c           | -            | 向标准输出写入字符 c                    |
| 0x56 | `print.f`    | -        | 1:f           | -            | 向标准输出写入浮点数 f                  |
| 0x57 | `print.s`    | -        | 1:addr, 2:l   | -            | 向标准输出写入地址 addr 的 l 长度字符串 |
| 0x58 | `println`    | -        | -             | -            | 向标准输出写入一个换行                  |
| 0xfe | `panic`      |          |               |              | 恐慌（导致强行退出）                    |

### `cmp.T` 指令

指令会在 `lhs < rhs` 时压入 `-1`, `lhs > rhs` 时压入 `1`, `lhs == rhs` 时压入 `0`。浮点数无法比较时压入 `0`。

### `load.8/16/32/64` 指令

指令会从 `addr` 处取 `T` 长度的数据压入栈中。如果 `addr` 不是 `T` 的倍数，将会产生 `UnalignedAccess` 错误。如果 `T` 小于 64，多余的数位将会被补成 0。

### `store.8/16/32/64` 指令

指令会将 `T` 长度的数据弹栈并存入 `addr` 地址处。如果 `addr` 不是 `T` 的倍数，将会产生 `UnalignedAccess` 错误。如果 `T` 小于 64，数据将被截断至 `T` 长度。
