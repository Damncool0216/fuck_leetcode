#![allow(dead_code, unused)]
struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let (mut l, mut r, mut sum, mut ans) = (0, 0, 0, 0);
        let len = nums.len();
        let want = nums.iter().sum::<i32>() - x;
        if want == 0 {
            return len as i32;
        }
        while r < len {
            sum += nums[r];
            r += 1;
            while sum > want && l < r {
                sum -= nums[l];
                l += 1;
            }
            if sum == want && r - l > ans {
                ans = r - l;
            }
        }
        if ans == 0 {
            -1
        } else {
            (len - ans) as i32
        }
    }
}
