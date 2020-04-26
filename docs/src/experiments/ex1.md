# 第一次实验 | 词法分析

嗨！如果你看到这里，想必你已经准备好来做第一个实验了。在这个实验里，我们要写一个词法分析器（一般称作 lexer 或者 tokenizer，本文使用 lexer），把 r0 语言的程序分割成一个一个的单词（token）。词法分析（lexing 或 tokenizing）的结果将作为语法分析（parsing）部分的输入进行进一步的处理。

让我们开始吧。

## 实验目标

本次实验的目标是：

- 构造一个词法分析器
- 实现词法分析的错误处理

## 实验指导

词法分析是程序编译的第一步。一般来说，一个编程语言的语法是建立在单词上的，而词法分析的任务就是把输入进来的文本形式的程序分割成一个个单词。单词的语法通常来说都比较简单，差不多在正则表达式级别。

下面，我们以分析一种简单的语言为例讲解词法分析器的写法。

### 准备工作

我们的示例词法分析器最终形态大概是这样的：

```java
public class Lexer {
    /// 获取下一个 Token
    public Token nextToken();

    // 辅助方法们
    /// 获取下一个字符
    private char nextChar();
    /// 预览下一个字符，是否使用取决于你的喜好
    private char peekNextChar();

    // 各种语法成分的分析器
    private Token lexNumber();
    private Token lexOp();
    private Token lexIdent();
    // ...
}
```

用作示例的语言则是学长学姐们的老朋友 —— miniplc0。不过，这次不需要写它的编译器了。

### 

## 实验内容


