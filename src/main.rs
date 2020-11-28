use std::io;
use rand::Rng;
use std::cmp::Ordering;

/**
*猜数字游戏
*/
fn main() {
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
            /// _为通配符
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
