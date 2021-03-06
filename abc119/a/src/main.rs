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
    "2019/04/30" => "Heisei",
    "2019/11/01" => "TBD"
}

fn main() {
    println!("{}", solve(&stdin!()));
}

fn solve(src: &str) -> String {
    input! {
        source = src,
        S: String 
    }

    let mut ans = "Heisei";
    if S.as_str() > "2019/04/30" {
        ans = "TBD"
    }

    return ans.to_string();
}
