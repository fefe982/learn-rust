// https://leetcode.com/problems/minimum-window-substring/
// 76. Minimum Window Substring
pub struct Solution;
impl Solution {
    fn to_idx(c: u8) -> usize {
        match c {
            b'a'..=b'z' => (c - b'a') as usize,
            b'A'..=b'Z' => (c - b'A') as usize + 26,
            _ => 52,
        }
    }
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut vec_t = vec![0; 52];
        for &c in t {
            vec_t[Self::to_idx(c)] += 1;
        }
        let mut vec_s = Vec::with_capacity(s.len() + 1);
        let mut vec_sc = vec![0; 52];
        for &c in s {
            vec_s.push(vec_sc.clone());
            vec_sc[Self::to_idx(c)] += 1;
        }
        vec_s.push(vec_sc);
        let mut min_len = usize::MAX;
        let mut min_start = 0;
        let mut start = 0;
        let mut end = 1;
        let mut cur_c = 0;
        while end <= s.len() {
            while cur_c < 52 && vec_t[cur_c] <= vec_s[end][cur_c] - vec_s[start][cur_c] {
                cur_c += 1;
            }
            if cur_c >= 52 {
                loop {
                    let c = Self::to_idx(s[start]);
                    if vec_t[c] < vec_s[end][c] - vec_s[start][c] {
                        start += 1;
                    } else {
                        break;
                    }
                }
                let l = end - start;
                if l < min_len {
                    min_len = l;
                    min_start = start;
                }
            }
            end += 1;
        }
        if min_len == usize::MAX {
            String::from("")
        } else {
            std::str::from_utf8(&s[min_start..(min_start + min_len)])
                .unwrap()
                .to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_window() {
        assert_eq!(
            Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC")),
            String::from("BANC")
        );
        assert_eq!(
            Solution::min_window(String::from("a"), String::from("a")),
            String::from("a")
        );
        assert_eq!(
            Solution::min_window(String::from("a"), String::from("aa")),
            String::from("")
        );
    }
}
