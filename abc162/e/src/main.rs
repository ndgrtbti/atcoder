#![allow(non_snake_case)]

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

macro_rules! stdin {
    () => {{
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
}

macro_rules! test {
    ($($input:expr => $expected_output:expr),*) => {
        #[test]
        fn solve_test() {
            let mut i = 0;
            println!("");
            $(
                i += 1;
                println!("Case {}:", i);
                println!("[in]\n{}", $input);
                println!("[out]\n{}", solve($input));
                println!("[expected out]\n{}", $expected_output);
                println!("");
                assert_eq!(solve($input), $expected_output);
             )*
        }
    }
}

test! {
r"3 2
" => "9",

r"3 200
" => "10813692",

r"100000 100000
" => "742202979"
}

use std::cmp::max;
use std::cmp::min;

fn main() {
    println!("{}", solve(&stdin!()));
}

fn solve(src: &str) -> String {
    input! {
        source = src,
        N: u64, K: u64
    }
    let MOD: u64 = 10u64.pow(9u32) + 7;

    let mut gcd_mod_cnt: Vec<u64> = vec![0; (K+1) as usize];
    let mut mod_sum: u64 = 0;
    for gcd in (1..K+1).rev() {
        let mod_cnt: u64 = (mod_pow(K/gcd, N, MOD) + MOD - over_mod_cnt(&gcd_mod_cnt, gcd, MOD)) % MOD;
        mod_sum += gcd*mod_cnt;
        mod_sum %= MOD;
        gcd_mod_cnt[gcd as usize] = mod_cnt;
    }

    let ans: String = mod_sum.to_string();
    return ans;
}

fn over_mod_cnt(gcd_mod_cnt: &Vec<u64>, gcd: u64, mod_val: u64) -> u64 {
    let mut mod_cnt: u64 = 0;
    let mut mul: u64 = gcd + gcd;
    while mul < gcd_mod_cnt.len() as u64 {
        mod_cnt += gcd_mod_cnt[mul as usize];
        mod_cnt %= mod_val;
        mul += gcd;
    }
    return mod_cnt;
}

#[test]
fn over_mod_cnt_test() {
    assert_eq!(5, over_mod_cnt(&vec![0, 0, 3, 1, 1], 1, 7));
}

fn mod_pow(base: u64, exp: u64, modval: u64) -> u64 {
    let mut pow = 1;
    let mut sqrt = base;
    let mut left_exp = exp;
    while left_exp > 0 {
        if left_exp & 0b1 == 0b1 {
            pow = pow * sqrt % modval
        }
        left_exp >>= 1;
        sqrt = sqrt * sqrt % modval;
    }
    return pow;
}

#[test]
fn mod_pow_test() {
    assert_eq!(1, mod_pow(1, 5, 2));
    assert_eq!(2, mod_pow(2, 4, 7));
    assert_eq!(4, mod_pow(4, 4, 6));
}
