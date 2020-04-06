# 编译原理评测姬说明

如果时间足够&&安排得开的话，编译原理课程将使用自动评测。以下为评测方案草案：

## 自建 Gitlab 存储库

此处未定。可能会在校内自建一个 Gitlab 的站点用于实验作业的存储与自动评测。

## `judge.toml` 与自动评测

为了简化评测步骤，我们将要求所有上交的作业都包含一个名为 `judge.toml` 的文件，作为评测机的配置文件。这个文件所在的目录将被视作自动评测时的根目录。

`judge.toml` 的文件内容如下所示：

```toml
# 你的学号
id = "17370000"

# 评测部分
[judge]

# 编译和评测你的程序时使用的 docker 环境镜像，不改变可以不填（默认是 `lazymio/compilers-env`）
image = {
    # 来源
    source = "hub",
    # 镜像名称
    image = "rustlang/rust:nightly"
}

# 编译编译器所需要依次执行的命令
# 所有命令执行的根目录都是 `judge.toml` 所在的目录
# 请将用到的每一个命令部分作为单独的字符串填入数组
build = [
    ["cargo", "fetch"],
    ["cargo", "build", "--release"]
]

# 在编译完成之后，运行编译器所需要的命令
# 运行环境是 linux
lex = ["./target/release/natrium", "lex", "$file"]
parse = ["./target/release/natrium", "parse", "$file"]
compile = ["./target/release/natrium", "compile", "$file", "--out", "$output"]
```
