pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn reverseV(input: &str) -> String {
    if "" == input {
        String::new();
    }
    let mut char_vec: Vec<String> = input.chars()
        .map(|c| c.to_string()).collect();
    char_vec.reverse();
    char_vec.join("")
}
