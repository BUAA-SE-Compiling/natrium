# 函数和全局变量

## 函数

```
function_param -> 'const'? IDENT ':' ty
function_param_list -> function_param (',' function_param)*
function -> 'fn' IDENT '(' function_param_list? ')' '->' ty block_stmt
//               ^~~~      ^~~~~~~~~~~~~~~~~~~~          ^~ ^~~~~~~~~~
//               |              |                        |  |
//               function_name  param_list     return_type  function_body
```

与 miniplc0 不同，c0 中存在函数。

c0 中一个函数的定义由 _函数名_、_参数列表_、_返回值_ 和 _函数体_ 组成。

函数的参数声明和变量声明类似，等价于在函数体内进行变量声明，只不过会保证已经初始化过。

函数体的组成单位是语句，见 [语句页面](stmt.md)。

## 全局变量

全局变量的声明与局部变量相同，都是使用 [声明语句](stmt.md#声明语句) 进行声明。
