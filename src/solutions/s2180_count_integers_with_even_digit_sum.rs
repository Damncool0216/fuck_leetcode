#![allow(dead_code, unused)]
struct Solution;
impl Solution {
    pub fn count_even(num: i32) -> i32 {
        (1..=num)
            .filter(|x| {
                let mut x = *x;
                let mut sum = 0;
                while x > 0 {
                    sum += x % 10;
                    x /= 10;
                }
                sum % 2 == 0
            })
            .count() as i32
    }
}
