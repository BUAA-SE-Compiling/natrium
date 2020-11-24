fn move(level: int, a: int, c: int) -> void {
    putint(level);
    putchar(32);
    putchar(a);
    putchar(32);
    putchar(c);
    putln();
}

fn hanoi(level: int, a: int, b: int, c: int) -> void {
    if level == 1 {
        move(level, a, c);
    } else {
        hanoi(level-1, a, c, b);
        move(level, a, c);
        hanoi(level-1, b, a, c);
    }
}

fn main() -> void {
    hanoi(6, 65, 66, 67);
}
