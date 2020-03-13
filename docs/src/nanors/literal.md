
# 字面量

## 数字

```
hex_number -> "0x" [0-9A-Fa-f]+
dec_number -> [0-9]+
sign -> [+-]

integer -> hex_number | dec_number
```
 

```
fractional_part -> "." dec_number
signed_dec_number -> sign? dec_number
exponential_part -> "e" signed_dec_number

float_number -> 
    | signed_dec_number fractional_part? exponential_part?
    | fractional_part exponential_part?
```

### 字符和字符串

```
escape_sequence -> 
    | "\\n"
    | "\\r"
    | "\\t"
    | "\\\""
    | "\\\'"
    | "\x" hex_number{2}
non_escaped_char -> [^\\\n\r"']
char -> non_escaped_char | escape_sequence

char_literal -> "\'" char "\'"
string_literal -> "\"" char* "\""
```
