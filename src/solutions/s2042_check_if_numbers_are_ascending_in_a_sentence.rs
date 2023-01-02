#![allow(dead_code, unused)]
pub struct Solution;
impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut last_num = -1;
        for e in s.split_whitespace() {
            if let Ok(n) = e.parse::<i32>() {
                if n <= last_num {
                    return false;
                }
                last_num = n;
            }
        }
        true
    }
}