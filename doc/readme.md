# AtCoder Rust

今回はrustの理解も兼ねて
[AtCoder Beginners Selection](https://atcoder.jp/contests/abs)を解いてみます.

自分が水色なので、そのレベルの知識があれば、解くのは難しくないので、
問題を解くことではなく、rustの文法に慣れることを目指して解説していきます.

- 文字列の扱い
- 標準入力の対応
- 実際に解いてみる.
- マクロを使い標準入力の改善
- 他の人でいい解答がないか調査.

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
引数は必ず文字列型になるため、AtCoder用には条件に応じて型を変換する必要があります.

```rust
let mut s = String::new();
std::io::stdin().read_line(&mut s).ok();
```
AtCoderでそのまま読むには[これ](https://qiita.com/penguinshunya/items/cd96803b74635aebefd6)が一番素直なやり方かなと思います.

前置きが相当長くなったので、この辺から真面目にAtCoderときましょう.

今回は上のサイトにある`read`のコードを使いました.また,全てを使うわけではないので、何も考えずにコンパイルするとwarningが出ます.
なので、コードの先頭に`#![allow(dead_code)]`と書くことを推奨しておきます.


### ABC081B

問題はループの中で再帰的に2で割っていけば,求まると思います.
rustの場合,条件を満たすまでずっとループするのは
`loop`でかけます.脱出の時はbreakですが、今回のようにforでネストしてしまう場合はスコープを指定する必要があります.

```rust
fn abc081() {
    let n: usize = read();
    let mut A: Vec<i32> = read_vec();
    let mut ret = 0;
    'outer: loop {
        for i in 0..n {
            if A[i] % 2 == 1 {
                break 'outer;
            } else {
                A[i] /= 2;
            }
        }
        ret += 1;
    };
    println!("{}", ret);
}
```

### ABC087B
1250通りしかないので、愚直に全探索します.
```rust
fn abc087b() {
    let a: i32 = read();
    let b: i32 = read();
    let c: i32 = read();
    let x: i32 = read();
    let mut num = 0;
    for _a in 0..(a+1) {
        for _b in 0..(b+1) {
            for _c in 0..(c+1) {
                if 500 *_a + 100 * _b + 50 * _c  == x {
                    num += 1;
                }
            }
        }
    }
    println!("{}", num);
}
```

### ABC083B
これも10000個しかないので、全探索します.
桁数の和は数値を文字列に変形して,一文字ずつ分割した後足しました.
rustの場合はchar型の列にして、それを文字コードをみながら整数に置き直しています.
```rust
fn abc083b(){
    let input: Vec<i32> = read_vec();
    let (n, a, b) = (input[0], input[1], input[2]);
    let mut num = 0;
    for _n in 1..(n+1) {
        let _n_sum: i32 = _n.to_string().chars().map(|c| c as i32 - 48).fold(0, |sum, c| sum + c);
        if _n_sum >= a  && _n_sum <= b {
            num += _n;
        }
    }
    println!("{}", num);
}
```

### ABC088B
全体をソートして差分をひきます.
```rust
fn abc088b() {
    let n: usize = read();
    let mut a: Vec<i32> = read_vec();
    let mut num = 0;
    a.sort();
    for _n in 0..n {
        if _n % 2 == 0 {
           num += a[n-1-_n];
        } else {
            num -= a[n-1-_n];
        }
    }
    println!("{}", num);
}
```
### ABC085B
重複排除して、サイズを数えればよいですね。
重複排除するときはsetにするのが自然かと思いますが、
rustの場合はHashSetクラスが存在するので、それで計算します.

```rust
fn abc085b(){
    let n: u32 = read();
    let d: Vec<Vec<u32>> = read_vec2(n);
    let uniq_d: HashSet<u32> = d.into_iter().map(|x| x[0]).collect();
    println!("{}", uniq_d.len());
}
```

### ABC085C
条件を考えると
1000円札をz枚使ったとき,
10000円札をx枚、5000円札をy枚使う解答があるかどうかは
- Y - 1000 z が5000で割り切れ
以下の方程式について、x,yを解くと0以上の整数解(今回は整数には必ずなので、)になればよいです。

- 2x + y = (Y - 1000 z) / 5000
- x + y =  N - z

それは x = (Y -1000 z) / 5000  - N + z >= 0かつ
y = N - z - x >= 0で判定できます.



実際、↓を読めばよくわかると思います.
- https://qiita.com/tubo28/items/e6076e9040da57368845
- https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8

### ABC049C
貪欲に解けばいいので、

1文字目がe 
  - eraser or erase
  - 文字の長さが5かつeraseならYes
  - 文字の長さが6かつeraserならYes
  - 文字列の長さが6以下で上記以外の場合No
  - 6文字目がrならeraserを削除
  - 6文字目がe or dなら eraseを削除
  - それ以外No
- dreamの場合も同様に分けていけばできます.
ただし、dreamera,dreamerd等で区別する必要はあります.


### ABC086 C
移動時間が最短移動時間＋2n(n >= 0)であればよいので、
それに合わせて実装します.

```rust
fn abc086c(){
    let n: u32 = read();
    let input: Vec<Vec<i32>> = read_vec2(n);
    let mut old_x = 0;
    let mut old_y = 0;
    let mut old_t = 0;
    let mut flag = true;
    for inp_i in input {
        let (t, x, y) = (inp_i[0], inp_i[1], inp_i[2]);
        let diff = t - old_t - (x - old_x).abs() - (y - old_y).abs();
        if diff < 0 || diff % 2 == 1 {
            println!("No");
            flag = false;
            break;
        } else{
            old_t = t;
            old_x = x;
            old_y = y;
        }
    }
    if flag {
        println!("Yes");
    }
}
```

### よりよいrustにするために


初めてやって思ったこと

- 文字列の扱いが難しい.
- Result型にするのは面白い.
- 配列のindexがuintでないとエラーになるのはつらい.
- numpyはすごいなという気持ちに...
  - (np.where含め集合的な処理ができたが、rustのndarrayではできず.)