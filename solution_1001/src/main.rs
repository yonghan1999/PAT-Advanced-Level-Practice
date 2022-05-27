use std::io::stdin;

struct Solution {}

impl Solution {
    pub fn a_add_b_format(num1: i32, num2: i32) -> String {
        let mut sum = num1 + num2;

        if sum == 0 {
            return "0".to_string();
        }
        let fs = sum < 0;
        sum = sum.abs();
        let mut stack = Vec::new();
        let mut i = 0;
        while sum != 0 {
            stack.push(char::from_u32((sum % 10 + 48) as u32).unwrap());
            sum /= 10;
            i = i + 1;
            if i % 3 == 0 && sum != 0 {
                stack.push(',');
            }
        }
        if fs {
            stack.push('-');
        }
        stack.reverse();
        let mut res = String::new();
        res.extend(stack.iter());
        return res;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace();
    let num1: i32 = nums.next().unwrap().parse().unwrap();
    let num2: i32 = nums.next().unwrap().parse().unwrap();
    let res = Solution::a_add_b_format(num1, num2);
    println!("{}", res);
}
