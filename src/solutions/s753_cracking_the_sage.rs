#![allow(dead_code, unused)]
struct Solution;
impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let n = n as u32;
        let (edgenum, nodenum) = (k.pow(n), k.pow(n - 1));
        let mut ans = String::with_capacity((edgenum + k - 1) as usize);
        let mut node = vec![k - 1; nodenum as usize];
        let mut idx = 0;
        for _ in (0..edgenum) {
            let x = node[idx];
            ans.push_str(x.to_string().as_str());
            node[idx] -= 1;
            idx = ((idx as i32 * k + x) % nodenum) as usize;
        }

        "0".repeat(n as usize - 1) + &ans
    }
}
