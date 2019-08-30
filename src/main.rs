#![allow(dead_code)]
use std::collections::HashSet;

fn main() {
    let mode = "abc081";
    if mode == "prcatice" {
        practice();
    } else if mode == "086" {
        abc086a();
    } else if mode == "abc081" {
        abc081a();
    } else if mode == "abc081b" {
        abc081b();
    } else if mode == "abc087b" {
        abc087b();
    } else if mode == "abc083b" {
        abc083b();
    } else if mode == "abc088b" {
        abc088b();
    } else if mode == "abc085b" {
        abc085b();
    } else if mode == "abc085c" {
        abc085c();
    } else if mode == "abc049c" {
        abc049c();
    } else if mode == "abc086c" {
        abc086c();
    }
}

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


fn abc049c(){
    let mut s: String = read();
    loop {
        let len = s.len();
        if len < 5 {
            println!("NO");
            break
        }
        if s == "erase" || s == "eraser" || s == "dream" || s == "dreamer" {
            println!("YES");
            break
        }
        if s.chars().nth(0).unwrap() == 'e' {
            if &s[0..6] == "eraser" {
                s = s[6..len].to_string();
            } else if &s[0..5] == "erase" {
                s = s[5..len].to_string();
            } else {
                println!("NO");
                break
            }
        } else if s.chars().nth(0).unwrap() == 'd' {
            if &s[0..7] == "dreamer" {
                if &s[0..8] == "dreamera" {
                    s = s[5..len].to_string();
                } else {
                    s = s[7..len].to_string();
                }
            } else if &s[0..5] == "dream" {
                s = s[5..len].to_string();
            } else {
                println!("NO");
                break
            }
        } else {
            println!("NO");
            break
        }
    }

}

fn abc085c(){
    let input: Vec<i32> = read_vec();
    let (n, Y) = (input[0], input[1]);
    let mut flag = false;
    for z in 0..(n+1) {
        if (Y - 1000 * z) % 5000 != 0 { continue }
        let div_y = (Y - 1000 * z) / 5000;
        let x = div_y - n + z;
        let y = n - z - x;
        if x < 0 || y < 0 { continue }
        flag = true;
        println!("{} {} {}", x, y, z);
        break
    }
    if !flag {
        println!("{} {} {}", -1, -1, -1);
    }
}

fn abc085b(){
    let n: u32 = read();
    let d: Vec<Vec<u32>> = read_vec2(n);
    let uniq_d: HashSet<u32> = d.into_iter().map(|x| x[0]).collect();
    println!("{}", uniq_d.len());
}



fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let v = s.trim().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect();
        v2.push(v);
    }
    v2
}

 
fn practice() {
    let first_int: Vec<i32> = read_vec();
    let second_int: Vec<i32> = read_vec();
    let s: String = read();
    let mut res = first_int[0];
    res += second_int[0] + second_int[1];
    println!("{} {}", res, s);
}

fn abc086a() {
    let fline: Vec<i32> = read_vec();
    if (fline[0] * fline[1]) % 2 == 1 {
        println!("Odd");
    } else {
        println!("Even");
    }
}

fn abc081a() {
    let s: String = read();
    let mut num = 0;
    for i in 0..3 {
        if s.chars().nth(i).unwrap() == '1' {
            num += 1;
        }
    }
    println!("{}", num);

}

fn abc081b() {
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