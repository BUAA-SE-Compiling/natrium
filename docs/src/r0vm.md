# r0 虚拟机标准

本次大作业的编译目标是 r0 虚拟机 (r0vm) 的汇编 (s0)。其设计参考了 JVM、DotNet CLR 和上学期的 c0 虚拟机。

## 虚拟机简介

r0vm 是一个具有线性内存空间的栈式虚拟机。作为一个实验用的虚拟机，r0vm 没有添加任何优化。

## 内存空间

r0vm 的内存空间以 8 位（1 字节）为单位寻址。8、16、32、64 位的数据类型分别以其大小为单位在内存中对齐。当读取或写入操作未对齐时，会产生 `UnalignedAccess` 错误。

r0vm 的栈空间以 8 字节为一个 slot，压栈、弹栈以及各种运算操作均以 slot 为单位进行。默认情况下，栈的大小是 1 MiB (1048576 字节)，即 131072 个 slot。栈空时弹栈和栈满时压栈分别会产生 `StackUnderflow` 和 `StackOverflow` 错误。

> 以 r0vm 为编译目标的话没有必要了解地址空间的结构，写在这里只是作为笔记使用。

r0vm 的全局变量、堆和栈在同一个线性地址空间内。地址的宽度是 64 bit。其中

- 全局变量从地址空间最底端 `0x0000_0000_0000_0000` 向上延伸至（不含） `0x0000_0001_0000_0000`
- 栈从地址空间最顶部 `0xffff_ffff_ffff_ffff` 向下延伸到（含） `0xffff_ffff_fff0_0000`
- 堆从地址 `0x0000_00001_0000_0000` 向上延伸。

地址空间示意图：

```
|--------------| <- 0xffff_ffff_ffff_ffff
| 栈   |       |
|      V       |    ↕ 1 MiB
|--------------| <- 0xffff_ffff_fff0_0000
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

r0vm 在运算中支持三种基本数据类型，分别是 64 位无符号整数 `u64`、64 位有符号整数 `i64`、64 位浮点数 `f64`。

`u64` 和 `i64` 都是 64 位整数，使用[二进制补码](2s_complement)形式表示。两种类型在多数整数运算中不做区分，仅在 `cmp.T`（比较指令，见下）等两种运算结果有差别的地方有所区分。在运算溢出时，两种类型均采用环绕 (wrap-around) 方式处理结果。

`f64` 是符合 [IEEE 754](ieee754) 规定的[双精度浮点数](double)。

[2s_complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[ieee754]: https://en.wikipedia.org/wiki/IEEE_754
[double]: https://en.wikipedia.org/wiki/Double-precision_floating-point_format

## 二进制格式

> TODO: 格式还没确定

s0 是 r0vm 所使用的汇编文件格式，其作用和内容类似 Java 的 `.class` 文件或者 DotNet 的 `.dll` 文件。

下面的结构体表示了 s0 的二进制文件结构。其中 `[T]` 表示类型 `T` 为元素的数组，在二进制文件中表示为 `u16` 大小的元素数量紧跟着所有元素的紧凑排列。

```rust
/// 整个 S0 二进制文件
struct S0 {
    /// 魔数
    magic: u32 = 0x72303b3e,
    /// 版本号，定为 1
    version: u32 = 0x00000001,
    /// 全局变量池
    globals: [GlobalDef],
    /// 函数池
    functions: [FunctionDef],
}

/// 单个常量
union GlobalDef {
    /// 二进制块（包括字符串）
    Blob {
        tag: u8 = 3,
        payload: [u8],
    },
}

struct FunctionDef {
    name: u16,
    param_slots: u16,
    return_slots: u16,
    max_stack: u16,
    body: [u8],
}
```

## 栈帧结构

```
| ...          |
|              | <- 栈顶 %sp
| 表达式栈 ... |
| 表达式栈     |
| 虚拟机参数...| 
| 虚拟机参数   | <- 被调用者栈底 %bp
| 局部变量 ... |
| 局部变量     | <- 调用者调用前 %sp
| 调用参数 ... | 
| 调用参数     | 
|--------------| ===
| 返回值       | v 
| 中间结果     | 调用者栈
| ...          | ^
|--------------| ===
```

其中，调用参数和返回值由调用者压栈，调用参数由被调用者清理。

### 虚拟机参数

虚拟机会在表达式栈和局部变量之间插入一系列的虚拟机参数以辅助虚拟机运行，目前暂定格式为（从栈顶到栈底）：

```
| ...           |
| 表达式栈      | 
|---------------|
| 调用者函数 ID | 
| 调用者 %ip    | 
| 调用者 %bp    | 
|---------------|
| 局部变量      | 
| ...           |
```

### 函数调用时栈帧变化示例

假设现有一个函数 `test`，有 1 slot 的返回值、2 slot 的参数和 2 slot 的局部变量。

```rust
/// 可以看成是这样的一个函数
fn test(a: i64, b: i64) -> i64 {
    let c: i64 = ...;
    let d: i64 = ...;
    ...
    return ...;
}
```

现在，它被编号为 1 的函数 `main` 调用，则执行 `call` 指令前的栈如下（不规定参数压栈顺序）：

```
| -          |
|------------| <- 栈顶
| b          | ^
| a          | 参数
| _ret       | 返回值
| ...        | ...表达式栈
```

在执行 `call` 指令后，栈如下：

```
| -          | <- 栈顶（表达式栈）
| 1          | ^
| %ip        | |
| %bp        | 虚拟机数据
| d          | ^
| c          | 局部变量
|------------|
| b          | ^
| a          | 参数
| _ret       | 返回值
| ...        |
```

在函数调用返回后，栈如下：

```
| -          | 
| // 1       | 
| // %ip     | 
| // %bp     |  ^
| // d       |  |  
| // c       |  |
| // b       |  |
| // a       | 以上内容被弹栈

|------------| <- 栈顶
| _ret       | 返回值
| ...        |
```

## 指令集

r0vm 的指令使用 8 位无符号整数标识，后面跟随可变长度的操作数。

下表展示了 r0vm 的所有指令。其中弹栈和压栈的格式为：`栈变化范围[:变量]`，数字按照栈底到栈顶编号。

| 指令 | 指令名       | 操作数   | 弹栈          | 压栈         | 介绍                                    |
| ---- | ------------ | -------- | ------------- | ------------ | --------------------------------------- |
| 0x00 | `nop`        | -        | -             | -            | 空指令                                  |
| 0x01 | `push`       | num:u64  | -             | 1,2:num      | 将 num 压栈                             |
| 0x02 | `pop`        | -        | 1             |              | 弹栈 1 个 slot                          |
| 0x03 | `popn`       | num:u32  | 1-num         | -            | 弹栈 num 个 slot                        |
| 0x04 | `dup`        | -        | 1:num         | 1:num, 2:num | 复制栈顶 slot                           |
| 0x08 | `loca`       | off:u32  | -             | 1:addr       | 加载 off 偏移量处局部变量的地址         |
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
| 0x30 | `cmp.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较有符号整数 lhs 和 rhs 大小          |
| 0x31 | `cmp.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较浮点数 lhs 和 rhs 大小              |
| 0x?? | `cmp.u`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较无符号整数 lhs 和 rhs 大小          |
| 0x32 | `neg.i`      | -        | 1:lhs         | 1:res        | 对 lhs 取反                             |
| 0x33 | `neg.f`      | -        | 1:lhs         | 1:res        | 对 lhs 取反                             |
| 0x34 | `itof`       | -        | 1:lhs         | 1:res        | 把 lhs 从整数转换成浮点数               |
| 0x35 | `ftoi`       | -        | 1:lhs         | 1:res        | 把 lhs 从浮点数转换成整数               |
| 0x36 | `shrl`       | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs >>> rhs （逻辑右移）     |
| 0x40 | `bra`        | -        | 1:addr        |              | 无条件跳转到地址 `addr`                 |
| 0x41 | `br`         | off:i32  |               |              | 无条件跳转偏移 `off`                    |
| 0x42 | `bz`         | off:i32  |               | 1:test       | 如果 `test` 是 0 则跳转偏移 `off`       |
| 0x43 | `bnz`        | off:i32  |               | 1:test       | 如果 `test` 非 0 则跳转偏移 `off`       |
| 0x44 | `bl`         | off:i32  |               | 1:test       | 如果 `test` 小于 0 则跳转偏移 `off`     |
| 0x45 | `bg`         | off:i32  |               | 1:test       | 如果 `test` 大于 0 则跳转偏移 `off`     |
| 0x46 | `blz`        | off:i32  |               | 1:test       | 如果 `test` 小于等于 0 则跳转偏移 `off` |
| 0x47 | `bgz`        | off:i32  |               | 1:test       | 如果 `test` 大于等于 0 则跳转偏移 `off` |
| 0x49 | `call`       |
| 0x4a | `ret`        |
| 0x50 | `scan.i`     |
| 0x51 | `scan.c`     |
| 0x52 | `scan.f`     |
| 0x54 | `print.i`    |
| 0x55 | `print.c`    |
| 0x56 | `print.f`    |
| 0x57 | `print.s`    |
| 0x58 | `println`    |
| 0xfe | `panic`      |          |               |              | 恐慌（导致强行退出）                    |
| 0xff | `halt`       |          |               |              | 停机                                    |

### `cmp.T` 指令

指令会在 `lhs < rhs` 时压入 `-1i64`, `lhs > rhs` 时压入 `1i64`, `lhs == rhs` 时压入 `0i64`。浮点数无法比较时压入 `0`。

### `load.T` 指令



### `store.T` 指令
