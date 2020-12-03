pub fn square_of_sum(n: u32) -> u32 {
    //n * (n + 1) * (2 * n + 1) / 6
    // ((1..(i + 1)).fold(0, |sum, x| sum + x)).pow(2)
    (1..n + 1).sum::<u32>().pow(2)
    // let mut sum = 0 as u32;
    // for i in 1..n + 1 {
    //     sum += i;
    // }
    // sum * sum
    // unimplemented!("square of sum of 1...{}", n)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // (n * (n + 1) / 2).pow(2)
    // (1..(j + 1)).fold(0, |sum, x| sum + x.pow(2));
    let s: u32 = (1..n + 1).map(|x| x.pow(2)).sum();
    s
    // let mut sum: u32 = 0;
    // for i in 1..n + 1 {
    //     sum += i * i;
    // }
    // sum
}

pub fn difference(n: u32) -> u32 {
    // let mut sum = 0 as u32;
    // for i in 1..n {
    //     for j in i + 1..n + 1 {
    //         sum += 2 * i * j;
    //     }
    // }
    // sum
    square_of_sum(n) - sum_of_squares(n)
}

pub fn square(s: u32) -> u64 {
    // match s {
    //     1...64 => 1u64.wrapping_shl(s-1),
    //     _ => panic!("Square must be between 1 and 64"),
    // }

    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    (2 as u64).pow(s - 1)
    // unimplemented!("grains of rice on square {}", s);
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
    // (2 as u64).pow(64)
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) {
    if factors.is_empty() {
        return;
    }

    (1..limit)
        .filter(|x| {
            for a in factors {
                if *a != 0 {
                    if x % *a == 0 {
                        return true;
                    }
                }
            }
            return false;
        })
        .for_each(|x| println!("{}", x));

    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
}
