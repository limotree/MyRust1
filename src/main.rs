mod gcd;
mod my_collect;
mod my_web;
mod string_exam;

use std::{io, thread};
use rand::Rng;
use std::cmp::Ordering;
use std::str::FromStr;
use std::time::Duration;
use std::collections::HashMap;
use gcd::command_line;
use gcd::guess_number;

fn main() {
    // 猜数字
    // guess_number();
    // 命令行
    // command_line();
    // my_collect::create_vec();
    let input = vec!["nail", "shoe", "horse"];
    // println!(
    //     "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n\
    //      7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n\
    //      6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
    // string_exam::build_proverb(&input) ;

        let mut r = String::new();

        for i in 0..3 {
            println!("{}",i);
            r.push_str(string_exam::verse(i).as_str());
        }




    println!("==={}",r);
}
