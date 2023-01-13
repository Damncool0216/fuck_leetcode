#![allow(dead_code, unused)]
struct Solution;
impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut mp = [0_i8; 11];
        num.bytes().for_each(|c| mp[(c - b'0') as usize] += 1);
        num.bytes().enumerate().all(|(i, c)| (c - b'0') as i8 == mp[i])
    }
}
