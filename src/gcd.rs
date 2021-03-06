use std::{io, thread};
use rand::Rng;
use std::cmp::Ordering;
use std::str::FromStr;
use std::time::Duration;
use std::collections::HashMap;

/// 计算最大公约数
///
/// ```
/// 可以cargo 执行， 也可以idea commond行配置执行
/// run  --package MyRust1 --bin MyRust1 42 56
/// ```
///
pub(crate) fn command_line() {
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
pub(crate) fn guess_number() {
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

fn ts_closure() {
    let generic_greeting = String::from("Good day,");
    let print_greeting = |name| println!("{} {}!", generic_greeting, name);
    let person = String::from("Crab");
    print_greeting(person);
    // println!("Can I use generic greeting? {}", generic_greeting);
    // println!("Can I use person {}", person);
}

fn th_thread() {
    let t1 = thread::spawn(|| {
        for i in 1..10 {
            println!("Greeting {} from other thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Greeting {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    t1.join().unwrap();
}