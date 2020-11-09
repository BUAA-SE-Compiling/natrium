const input = `
fn fib(x: int) -> int {
    if x<=1 {
        return 1;
    }
    let result:int = fib(x-1);
    result = result + fib(x-2);
    return result;
}

fn main() -> int {
    let i: int = 0;
    let j: int;
    j = getint();
    while i == i {
        putint(i);
        putchar(32);
        putint(fib(i));
        putln();
        i = i + 1;
        if i > j {
            break;
        }
    }
    return 0;
}`

import('../pkg/index.js')
  .catch(console.error)
  .then((module) => {
    console.log(module.compile(input))
  })
