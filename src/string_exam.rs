use std::iter::once;

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn reverseV(input: &str) -> String {
    if "" == input {
        String::new();
    }
    let mut char_vec: Vec<String> = input.chars().map(|c| c.to_string()).collect();
    char_vec.reverse();
    char_vec.join("")
}

fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32 + 1;

    (2..limit).all(|i| n % i != 0)
}

/// 计算素数
pub fn nth(n: u32) -> u32 {
    // (2..).filter(|&i| is_prime(i) ).nth(n as usize).unwrap()
    (2..).filter(|i| is_prime(*i)).nth(n as usize).unwrap()
}

pub fn build_proverb2(list: &[&str]) -> String {
    if list.is_empty() {
        return String::from("");
    }
    let mut r = String::new();
    for i in 0..list.len() - 1 {
        println!("len:{}", i);
        r.push_str("For want of a ");
        r.push_str(list[i]);
        r.push_str(" the ");
        r.push_str(list[i + 1]);
        r.push_str(" was lost.\n");
    }
    r.push_str("And all for the want of a ");
    r.push_str(list[0]);
    r.push_str(".");
    println!("{}", r);
    r
}

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|text| {
            format!(
                "For want of a {} the {} was lost.\n",
                text[0], text[1]
            )
        })
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect()
}

// pub fn verse(n: u32) -> String {
//
//     if n==0 {
//         return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
//     }
//     if n == 1 {
//         return format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n,n);
//     }
//     return format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, {} bottles of beer on the wall.\n", n,n,n-1);
// }

pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => format!(
            "{0} {1} of beer on the wall, {0} {1} of beer.\nTake one down and pass it around, {2} {3} of beer on the wall.\n",
            n, bottles(n), n-1, bottles(n-1)
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    // (end..start+1).rev().map(verse).collect::<Vec<_>>().join("\n")
    (end..start+1).rev().map(verse).collect::<Vec<String>>().join("\n")
    // let verses: Vec<String> = (end..start+1).rev().map(verse).collect().join("\n");
    // verses.join("\n")
}

fn bottles(n: u32) -> &'static str {
    match n {
        1 => "bottle",
        _ => "bottles"
    }
}