use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::str::FromStr;

fn main() {
    // guess_number();
    command_line();
}

/// 计算最大公约数
///
/// ```
/// 可以cargo 执行， 也可以idea commond行配置执行
/// run  --package MyRust1 --bin MyRust1 42 56
/// ```
///
fn command_line() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage:gcd Number...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for i in &numbers[1..] {
        d = gcd(d, *i);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}


///猜数字游戏
fn guess_number() {
    println!("Guess the number!");
    println!("Please input your guess!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("secret number: {}", secret_number);


    loop {
        /// 可变引用
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            // 通配符
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Greater than!"),
            Ordering::Equal => {
                println!("Yor win!");
                break;
            }
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        if b < a {
            let t = a;
            a = b;
            b = t;
        }
        b = b % a;
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
}
