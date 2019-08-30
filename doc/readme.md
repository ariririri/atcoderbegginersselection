# AtCoder Rust

[AtCoder Beginners Selection](https://atcoder.jp/contests/abs)を解いてみます.

- 文字列の扱い
- 標準入力の対応
- 実際に解いてみる.
- 1~10
- マクロを使い入出力を簡単にする.

## 文字列の扱い
競技プログラミングでは入力を受けて処理をするので、まず入力の文字列を処理する方法を考えます.

rustでは文字列を扱うときはよくstr型とString型が使われます.
Stringは[コード](https://github.com/rust-lang/rust/blob/master/src/liballoc/string.rs#L283)を見ると構造体としては
Vector型で,strはu8のスライスとなります.

どちらもutf8以外受け取らない様になっています.

Stringはヒープに値を持つの可変長,strは固定長なので,適当に文字列操作して新しい文字列を操作するのであれば,`&str`型を使うほうが性能が出そうです.
ただ,str型は基本さらに参照して使う事が多いため,メソッドの引数と復帰値両方に使う場合はライフタイムを意識することになります.
なので、新しい文字列を生成する場合は`String`型の方が良さそうに見えます.

1. strとStringの変換

str -> String: `to_string`,
String -> &str: `&a` &をつけるだけ(本当に型が変わるんだろうか?自動でキャストされる機能があったけか)?

2. メソッドでのライフタイム

ライフタイムは変数が何かを参照して作ったもの場合に,その参照先のデータが残っていることを保証するための仕組みです.
~rustではヒープに対してもスタックと同じでFILOの仕組みで値が保たれるようにしているのかな?~

とはいえ、常に書いていると面倒なので、基本はインタプリタ側で推論してくれています.
ただ,いつまでなのか期間が保証されない場合,例えば以下のメソッドを見ます.

```rust
fn main() {
    let hello = "Hello";
    let world = "World";
    println!("{}", longer(hello, world));
}

fn longer(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { return x} else {return y}
    }
```
この時,

```
|
9 | fn longer(x: &str, y: &str) -> &str {
 |                                ^ expected lifetime parameter
 |
 = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```
とエラーになります.

この変数の復帰値がどこまでのライフタイムを保証すればいいかわからないからです.
この場合はx,yの短い方と明記する必要があります.(実際これをライフタイムの短い方に勝手に推論してくれてもいい気はしますが...)
なので、引数の参照になる場合は気をつけましょう
これを回避するにはライフタイムを指定するのが常套手段ですが、
それ以外に以下のように新しくヒープ領域にデータを作ってしまう(つまりString)にしてしまうという方法があります.

```rust
fn main() {
    let hello = "Hello";
    let world = "World";
    println!("{}", longer(hello, world));
}

fn longer(x: &str, y: &str) -> String {
    if x.len() >= y.len() { return x.to_string()} else {return y.to_string()}
    }
```

こういうエラーははまりやすいので、メソッドの復帰値では新しい文字列をStringで生成する方が楽なのかなと思います.

### 標準入力

rustの標準入力はread_lineが使われます.
[ドキュメント](https://doc.rust-lang.org/std/io/struct.Stdin.html)を見ればわかるように,引数にした変数に入力をわたし、復帰値のResult型でうまくいったかどうかを表します.
引数は必ず文字列型になるため、AtCoder用には条件に応じて型を変換する必要があります.

```rust
let mut s = String::new();
std::io::stdin().read_line(&mut s).ok();
```
AtCoderでそのまま読むには[これ](https://qiita.com/penguinshunya/items/cd96803b74635aebefd6)が一番素直なやり方かなと思います.

前置きが相当長くなったので、この辺から真面目にAtCoderときましょう.



実際、↓を読めばよくわかると思います.
- https://qiita.com/tubo28/items/e6076e9040da57368845
- https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8


## わからないもの
-
