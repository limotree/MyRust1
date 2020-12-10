mod gcd;
mod my_collect;
mod my_math;
mod my_web;
mod string_exam;
mod list;

use gcd::command_line;
use gcd::guess_number;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use std::{io, thread};

fn main() {
    // 猜数字
    // guess_number();
    // 命令行
    // command_line();
    // my_collect::create_vec();
    let input = vec!["nail", "shoe", "horse"];
    // my_math::sum_of_multiples(15, &[4,6]);
    let mut f = 60;
    for j in 0..100 {
        for i in 2..60 {
            if f % i == 0 {
                f /= i;
                println!("f:{},i:{}", f, i);
                break;
            }
        }
    }
}
