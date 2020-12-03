mod gcd;
mod my_collect;
mod my_web;
mod string_exam;
mod my_math;

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
   my_math::sum_of_multiples(15, &[4,6]);


}
