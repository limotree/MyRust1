use std::collections::HashMap;

/// Vec 练习
pub(crate) fn create_vec() {
    let words = vec![1, 112, 33];
    let mut myV: Vec<i32> = Vec::new();
    let mut map = HashMap::new();

    for (i, word) in words.iter().enumerate() {
        println!("pos: {}, {}", i, word);
        myV.push(i as i32);
        myV.push(*word);
        map.insert(word, 1);
        myV.push(*map.get(word).unwrap())
    }
}


/// 计算两数之和，返回下标
pub fn two_sum(numbs: Vec<i32>, target: i32) -> Vec<i32> {
    let mut r: Vec<i32> = Vec::new();
    let mut map = HashMap::new();

    for (i, v) in numbs.iter().enumerate() {
        if map.contains_key(v) {
            r.push(*map.get(v).unwrap());
            r.push(i as i32);
            return r;
        }
        map.insert(target - v, i as i32);
    }
    r
}


