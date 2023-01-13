#![allow(dead_code, unused)]
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let (mut maps, mut mapt) = ([0_i8; 26], [0_i8; 26]);
        s.bytes().for_each(|c| maps[(c - b'a') as usize] += 1);
        target.bytes().for_each(|c| mapt[(c - b'a') as usize] += 1);
        let mut ans = i8::MAX;
        for (idx, &cnt) in mapt.iter().enumerate() {
            if cnt > 0 {
                if maps[idx] < cnt {
                    return 0;
                }
                ans = ans.min(maps[idx] / cnt);
            }
        }
        ans as i32
    }
}
