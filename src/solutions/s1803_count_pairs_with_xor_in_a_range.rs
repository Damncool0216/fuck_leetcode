#![allow(dead_code, unused)]
struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        let len = nums.len();
        let mut ans = 0;
        for i in 0..len {
            for j in (i + 1)..len {
                let xor = nums[i] ^ nums[j];
                if xor > low && xor < high {
                    ans += 1;
                }
            }
        }
        ans
    }
}
