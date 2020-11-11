# 实现方法指导

实现一个 parser 有很多种方法，这里会提供一些关于代码实现（而不是理论）的方法指导。

对于没有讲到的内容，可以参考去年的指导书：https://github.com/BUAA-SE-Compiling/c0-handbook#33-%E5%AE%9E%E7%8E%B0%E6%8C%87%E5%BC%95

## 一些通用的部分

### 类型

对于词法、语法分析时产生的类型，因为类型确定且已知，可以使用继承实现。在支持和类型 (sum type) 的语言里也可以用和类型实现。这样做可以显著降低判断 token 或者语法树节点类型时的工作量，因为可以直接判断变量本身的类型，甚至直接进行模式匹配。比如：

```csharp
/* 词法分析器 */

class Token {}

class NumberToken : Token {
    public double value;
}

// ...

/* 语法分析器 */

class Expr {}

class Literal : Expr {}

class IntegerLiteral : Literal {
    public long value;
}

class StringLiteral : Literal {
    public string value;
}

class BinaryExpr : Expr {
    public Operator op;
    public Expr lhs;
    public Expr rhs;
}

// ...
```

或者在支持和类型的语言里：

```rust
enum Expr {
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    // ...
}

enum LiteralExpr {
    Integer(i64),
    String(String),
    // ...
}

struct BinaryExpr {
    pub op: Operator,
    pub lhs: Ptr<Expr>,
    pub rhs: Ptr<Expr>,
}

// ...
```

### 迭代器

迭代器（Iterator）是对一系列值的抽象，比如说一列输入的字符或者解析完的 token。使用迭代器可以有效地将输入数据和对数据的获取操作解耦，方便在不同时候使用不同方式输入数据，以及进行测试。常见高级语言都有对于迭代器的抽象，包括：

- Java: `java.util.Iterator`
- C#: `System.Collections.Generic.IEnumerator`
- C++: `std::iterator::iterator_traits`
- C++20: concept `std::ranges::input_iterator`
- Python: 实现 `__next__` 的类型
- JavaScript: 实现 `Symbol.iterator` 的类型

由于在解析时常常要回溯，使用的迭代器可以提供一些额外的方法，比如 `peek()` 用于查看下一个值但不移动迭代器，或者 `unread(value)` 用于将已经读出的值放回迭代器。

## 词法分析

词法分析这个主题比较简单，基本上就是对于每个 token 使用自动机（或者退化成普通的逻辑分支）进行解析。token 的组成一般比较简单，可以在分析时参考正则表达式的状态。

比如我们要分析一个数字（整数或浮点数），我们可以写出它的正则表达式：`\d*\.\d*(e[+-]?\d+)?|0x[0-9a-fA-F]+|\d+`

> 不要学助教[用逻辑分支模拟自动机][bad_lexing]（逃

[bad_lexing]: https://github.com/01010101lzy/chigusa/blob/0a08176f4318542c1bb96114ac3f0df56ac9510d/src/c0/lexer.rs#L392-L511

## 语法分析

### 普通的递归下降分析法

递归下降是一个很简单、很直观的分析法，也是大多数人实现语法分析的首选方法。在实现递归下降分析器的时候，有一些可以降低编码难度的方法。

TODO: 展示一个一般的递归下降分析的过程，使用 C++ 或者类似语言

#### 使用迭代器

TODO: 迭代器是个好东西。展示使用迭代器解耦的方法

[error_recover]: https://www.eyalkalderon.com/nom-error-recovery/

#### 解析器结合子 (Parser Combinator)

TODO: 展示使用 Parser Combinator 的库，比如 nom，以及其使用方法

### 使用 LL/LR 解析器生成器

唔……这个咱似乎帮不上什么忙。记得选一个好用的，比如 [ANTLR][]。

[antlr]: https://www.antlr.org
