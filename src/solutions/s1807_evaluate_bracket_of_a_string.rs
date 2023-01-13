#![allow(dead_code, unused)]
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let map: HashMap<&str, &str> = knowledge
            .iter()
            .map(|v| (v[0].as_str(), v[1].as_str()))
            .collect();
        let mut s = s;
        s.push('(');
        let (mut l, mut r) = (0, 0);
        let mut ans = String::with_capacity(s.len());
        for c in s.bytes() {
            match c {
                b'(' => {
                    ans.push_str(&s[l..r]);
                    r += 1;
                    l = r;
                }
                b')' => {
                    if let Some(x) = map.get(&s[l..r]) {
                        ans.push_str(x);
                    } else {
                        ans.push_str("?");
                    }
                    r += 1;
                    l = r;
                }
                _ => r += 1,
            }
        }
        ans
    }
}
