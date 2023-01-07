# If

`if`, the most basic (but still surprisingly versatile!) type of control flow, is what you'll learn here.

## Further information

- [Control Flow - if expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

---
# memo
こんな漢字で早期return する場合は、明示的にreturn 書かないと行けない
```rs
fn a() -> $str {
    if (a > b) {
        // a のみではだめ
        return a;
    } 
    b
}
```
それかelse 句 書いて同じように書くか　になる
```rs
fn a() -> $str {
    if (a > b) {
        a
    } else {
        b
    } 
}
```