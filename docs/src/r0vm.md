# r0 虚拟机标准

本次大作业的编译目标是 r0 虚拟机 (r0vm) 的汇编 (s0)。其设计参考了 JVM、DotNet CLR 和上学期的 c0 虚拟机。

## 虚拟机简介

r0vm 是一个具有线性内存空间的栈式虚拟机。栈以外的内存空间以 8 位（1 字节）为单位寻址，栈空间以 64 位（8 字节）为单位寻址。

## 二进制格式

下面的结构体表示了 s0 的二进制文件结构。其中 `[T]` 表示类型 `T` 为元素的数组，在二进制文件中表示为 `u16` 大小的元素数量紧跟着所有元素的紧凑排列。

```rust
/// 整个 S0 二进制文件
struct S0 {
    /// 魔数
    magic: u32 = 0x72303b3e,
    /// 版本号，定为 1
    version: u32 = 0x00000001,
    /// 常量池
    globals: [GlobalDef],
    /// 函数池
    functions: [FunctionDef],
}

/// 单个常量
union GlobalDef {
    /// 32 位整数
    I32 {
        tag: u8 = 1,
        num: i32,
    },
    /// 64 位浮点数
    F64 {
        tag: u8 = 2,
        num: f64,
    },
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
| 中间结果     | 
| 虚拟机参数   | <- 被调用者栈底 %bp
| 局部变量     | 
| 调用参数     |
|--------------| ===
| 返回值       | v 
| 中间结果     | 调用者栈
| ...          | ^
|--------------| ===
```

其中，调用参数和返回值由调用者压栈，调用参数由被调用者清理。

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
| 0x08 | `loca`       | n:u32    | -             | 1:addr       | 加载第 n 个局部变量的地址               |
| 0x09 | `globa`      | n:u32    | -             | 1:addr       | 加载第 n 个全局变量（常量？）的地址     |
| 0x10 | `load.8`     | -        | 1:addr        | 1:val        | 从 addr 加载 8 位 value 压栈            |
| 0x11 | `load.16`    | -        | 1:addr        | 1:val        | 从 addr 加载 16 位 value 压栈           |
| 0x12 | `load.32`    | -        | 1:addr        | 1:val        | 从 addr 加载 32 位 value 压栈           |
| 0x13 | `load.64`    | -        | 1:addr        | 1:val        | 从 addr 加载 64 位 value 压栈           |
| 0x14 | `store.8`    | -        | 1:addr, 2:val | -            | 把 val 截断到 8 位存入 addr             |
| 0x15 | `store.16`   | -        | 1:addr, 2:val | -            | 把 val 截断到 16 位存入 addr            |
| 0x16 | `store.32`   | -        | 1:addr, 2:val | -            | 把 val 截断到 32 位存入 addr            |
| 0x17 | `store.64`   | -        | 1:addr, 2:val | -            | 把 val 存入 addr                        |
| 0x18 | `alloc`      | -        | 1:size        | 1:addr       | 在堆上分配 size 字节的内存              |
| 0x19 | `free`       | -        | 1:addr        | -            | 释放 addr 指向的内存块                  |
| 0x1a | `stackalloc` | size:u32 | -             | -            | 在栈上表达式区分配 size slot 的内存     |
| 0x20 | `add.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs + rhs，参数为整数        |
| 0x21 | `sub.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs - rhs，参数为整数        |
| 0x22 | `mul.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs * rhs，参数为整数        |
| 0x23 | `div.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs / rhs，参数为整数        |
| 0x24 | `add.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs + rhs，参数为浮点数      |
| 0x25 | `sub.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs - rhs，参数为浮点数      |
| 0x26 | `mul.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs * rhs，参数为浮点数      |
| 0x27 | `div.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs / rhs，参数为浮点数      |
| 0x28 | `adc.i`      | -        | 1:lhs, 2:rhs  | 1:res        | (待确认)                                |
| 0x29 | `shl`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs << rhs                   |
| 0x2a | `shr`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs >> rhs （算术右移）      |
| 0x2b | `and`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs & rhs                    |
| 0x2c | `or`         | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs &#124; rhs               |
| 0x2d | `xor`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs ^ rhs                    |
| 0x2d | `not`        | -        | 1:lhs         | 1:res        | 计算 res = !lhs                         |
| 0x30 | `cmp.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较整数 lhs 和 rhs 大小                |
| 0x31 | `cmp.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较浮点数 lhs 和 rhs 大小              |
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
| 0xff | `halt`       |          |               |              | 停机                                    |
