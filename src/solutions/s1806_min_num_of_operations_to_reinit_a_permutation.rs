#![allow(dead_code, unused)]
pub struct Solution;
impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        let (mut ac, mut ans) = (2, 1);
        while ac != 1 {
            ac = ac * 2 % (n - 1);
            ans += 1;
        }
        ans
    }
}
