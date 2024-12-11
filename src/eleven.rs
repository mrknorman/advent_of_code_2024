use rayon::prelude::*;
use cached::proc_macro::cached;

fn has_even_digits(num: u64) -> bool {
    (num as f64).log10() as u64 % 2 == 1
}

fn split_number(input: u64) -> (u64, u64) {
    let digits = (input as f64).log10() as u32 + 1;
    let half = digits / 2;
    let divisor = 10u64.pow(half);
    (input / divisor, input % divisor)
}

#[cached]
fn apply_rules(input: u64, index: usize, depth: usize) -> u64 {
    if index == depth {
        return 1;
    }

    if input == 0 {
        apply_rules(1, index + 1, depth)
    } else if has_even_digits(input) {
        let (first, second) = split_number(input);
        apply_rules(first, index + 1, depth) + apply_rules(second, index + 1, depth)
    } else {
        apply_rules(input * 2024, index + 1, depth)
    }
}

pub fn eleven_a() {
    let include: Vec<u64> = include_str!("../res/11_input.txt")
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let list_len: u64 = include
        .par_iter()
        .map(|&value| apply_rules(value, 0, 25))
        .sum();
        
    println!("Num stones: {:?}", list_len);
}

pub fn eleven_b() {
    let include: Vec<u64> = include_str!("../res/11_input.txt")
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let list_len: u64 = include
        .par_iter()
        .map(|&value| apply_rules(value, 0, 70))
        .sum();
        
    println!("Num stones: {:?}", list_len);
}