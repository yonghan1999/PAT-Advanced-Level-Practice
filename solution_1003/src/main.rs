use std::io::stdin;

struct Solution {}

impl Solution {
}

// 看懂了不会👎
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace();
    let n: i32 = nums.next().unwrap().parse().unwrap();
    let m: i32 = nums.next().unwrap().parse().unwrap();
    let c1: i32 = nums.next().unwrap().parse().unwrap();
    let c2: i32 = nums.next().unwrap().parse().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace();
    let teams = Vec::from_iter(nums);
    println!("{:?}",teams);
}
