# Primitive Types

Rust has a couple of basic types that are directly implemented into the
compiler. In this section, we'll go through the most important ones.

## Further information

- [Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [The Slice Type](https://doc.rust-lang.org/stable/book/ch04-03-slices.html)

---
# memo
https://qiita.com/deta-mamoru/items/e2262fcde20e8e3f97b9#%E3%82%B9%E3%83%A9%E3%82%A4%E3%82%B9

## 配列
配列は固定長で同じ型の要素を持つ型です。固定長のため拡大縮小や、サイズ以上の要素は持てません。

## ベクター
ベクタはヒープメモリに確保される可変の配列です。

## スライス
スライスは、配列やベクタなどのある位置を指し、[T]のように書きます。そして、常に参照されます。