# 指令集

navm 的指令使用 8 位（1 字节）无符号整数标识，后面跟随可变长度的操作数。操作数类型为 `u64` `i64` 时，长度为 64 位（8 字节），类型为 `u32` `i32` 时，长度为 32 位（4 字节）。

下表展示了 navm 的所有指令。其中弹栈和压栈的格式为：`栈变化范围[:变量]`，数字按照栈底到栈顶编号。

| 指令 | 指令名       | 操作数   | 弹栈          | 压栈         | 介绍                                     |
| ---- | ------------ | -------- | ------------- | ------------ | ---------------------------------------- |
| 0x00 | `nop`        | -        | -             | -            | 空指令                                   |
| 0x01 | `push`       | num:u64  | -             | 1:num        | 将 num 压栈                              |
| 0x02 | `pop`        | -        | 1             |              | 弹栈 1 个 slot                           |
| 0x03 | `popn`       | num:u32  | 1-num         | -            | 弹栈 num 个 slot                         |
| 0x04 | `dup`        | -        | 1:num         | 1:num, 2:num | 复制栈顶 slot                            |
| 0x08 | `loca`       | off:u32  | -             | 1:addr       | 加载 off 个 slot 处局部变量的地址        |
| 0x08 | `arga`       | off:u32  | -             | 1:addr       | 加载 off 个 slot 处参数/返回值的地址     |
| 0x09 | `globa`      | n:u32    | -             | 1:addr       | 加载第 n 个全局变量/常量的地址           |
| 0x10 | `load.8`     | -        | 1:addr        | 1:val        | 从 addr 加载 8 位 value 压栈             |
| 0x11 | `load.16`    | -        | 1:addr        | 1:val        | 从 addr 加载 16 位 value 压栈            |
| 0x12 | `load.32`    | -        | 1:addr        | 1:val        | 从 addr 加载 32 位 value 压栈            |
| 0x13 | `load.64`    | -        | 1:addr        | 1:val        | 从 addr 加载 64 位 value 压栈            |
| 0x14 | `store.8`    | -        | 1:addr, 2:val | -            | 把 val 截断到 8 位存入 addr              |
| 0x15 | `store.16`   | -        | 1:addr, 2:val | -            | 把 val 截断到 16 位存入 addr             |
| 0x16 | `store.32`   | -        | 1:addr, 2:val | -            | 把 val 截断到 32 位存入 addr             |
| 0x17 | `store.64`   | -        | 1:addr, 2:val | -            | 把 val 存入 addr                         |
| 0x18 | `alloc`      | -        | 1:size        | 1:addr       | 在堆上分配 size 字节的内存               |
| 0x19 | `free`       | -        | 1:addr        | -            | 释放 addr 指向的内存块                   |
| 0x1a | `stackalloc` | size:u32 | -             | -            | 在当前栈顶分配 size 个 slot，初始化为 0  |
| 0x20 | `add.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs + rhs，参数为整数         |
| 0x21 | `sub.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs - rhs，参数为整数         |
| 0x22 | `mul.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs * rhs，参数为整数         |
| 0x23 | `div.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs / rhs，参数为有符号整数   |
| 0x24 | `add.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs + rhs，参数为浮点数       |
| 0x25 | `sub.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs - rhs，参数为浮点数       |
| 0x26 | `mul.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs * rhs，参数为浮点数       |
| 0x27 | `div.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs / rhs，参数为浮点数       |
| 0x28 | `div.u`      | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs / rhs，参数为无符号整数   |
| 0x29 | `shl`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs << rhs                    |
| 0x2a | `shr`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs >> rhs （算术右移）       |
| 0x2b | `and`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs & rhs                     |
| 0x2c | `or`         | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs &#124; rhs                |
| 0x2d | `xor`        | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs ^ rhs                     |
| 0x2d | `not`        | -        | 1:lhs         | 1:res        | 计算 res = !lhs                          |
| 0x2e | `inv`        | -        | 1:lhs         | 1:res        | 计算 res = ~lhs（按位反转）              |
| 0x30 | `cmp.i`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较有符号整数 lhs 和 rhs 大小           |
| 0x31 | `cmp.f`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较浮点数 lhs 和 rhs 大小               |
| 0x32 | `cmp.u`      | -        | 1:lhs, 2:rhs  | 1:res        | 比较无符号整数 lhs 和 rhs 大小           |
| 0x34 | `neg.i`      | -        | 1:lhs         | 1:res        | 对 lhs 取反                              |
| 0x35 | `neg.f`      | -        | 1:lhs         | 1:res        | 对 lhs 取反                              |
| 0x36 | `itof`       | -        | 1:lhs         | 1:res        | 把 lhs 从整数转换成浮点数                |
| 0x37 | `ftoi`       | -        | 1:lhs         | 1:res        | 把 lhs 从浮点数转换成整数                |
| 0x38 | `shrl`       | -        | 1:lhs, 2:rhs  | 1:res        | 计算 res = lhs >>> rhs （逻辑右移）      |
| 0x39 | `set.lt`     | -        | 1:lhs         | 1:res        | 如果 lhs < 0 则推入 1，否则 0            |
| 0x3a | `set.gt`     | -        | 1:lhs         | 1:res        | 如果 lhs > 0 则推入 1，否则 0            |
| 0x41 | `br`         | off:i32  |               |              | 无条件跳转偏移 `off`                     |
| 0x42 | `br.false`   | off:i32  |               | 1:test       | 如果 `test` 是 0 则跳转偏移 `off`        |
| 0x43 | `br.true`    | off:i32  |               | 1:test       | 如果 `test` 非 0 则跳转偏移 `off`        |
| 0x48 | `call`       | id:u32   | 见栈帧介绍    | -            | 调用编号为 id 的函数                     |
| 0x49 | `ret`        | -        | -             | 见栈帧介绍   | 从当前函数返回                           |
| 0x4a | `callname`   | id:u32   | 见栈帧介绍    | -            | 调用名称与编号为 id 的全局变量相同的函数 |
| 0x50 | `scan.i`     | -        | -             | 1:n          | 从标准输入读入一个整数 n                 |
| 0x51 | `scan.c`     | -        | -             | 1:c          | 从标准输入读入一个字符 c                 |
| 0x52 | `scan.f`     | -        | -             | 1:f          | 从标准输入读入一个浮点数 f               |
| 0x54 | `print.i`    | -        | 1:x           | -            | 向标准输出写入一个有符号整数 x           |
| 0x55 | `print.c`    | -        | 1:c           | -            | 向标准输出写入字符 c                     |
| 0x56 | `print.f`    | -        | 1:f           | -            | 向标准输出写入浮点数 f                   |
| 0x57 | `print.s`    | -        | 1:i           | -            | 向标准输出写入全局变量 i 代表的字符串    |
| 0x58 | `println`    | -        | -             | -            | 向标准输出写入一个换行                   |
| 0xfe | `panic`      |          |               |              | 恐慌（强行退出）                         |

<!-- | 0x40 | `bra`        | -        | 1:addr        |              | 无条件跳转到地址 `addr`                 | -->

### `cmp.T` 指令

指令会在 `lhs < rhs` 时压入 `-1`, `lhs > rhs` 时压入 `1`, `lhs == rhs` 时压入 `0`。浮点数无法比较时压入 `0`。



### `load.8/16/32/64` 指令

指令会从 `addr` 处取 `T` 长度的数据压入栈中。如果 `addr` 不是 `T` 的倍数，将会产生 `UnalignedAccess` 错误。如果 `T` 小于 64，多余的数位将会被补成 0。

### `store.8/16/32/64` 指令

指令会将 `T` 长度的数据弹栈并存入 `addr` 地址处。如果 `addr` 不是 `T` 的倍数，将会产生 `UnalignedAccess` 错误。如果 `T` 小于 64，数据将被截断至 `T` 长度。
