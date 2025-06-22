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
        let mut vec_s = vec![0; 52];
        let mut good = 0;
        for i in 0..52 {
            if vec_s[i] >= vec_t[i] {
                good += 1;
            }
        }
        let mut min_len = usize::MAX;
        let mut min_start = 0;
        let mut start = 0;
        let mut end = 0;
        while end < s.len() {
            let iec = Self::to_idx(s[end]);
            vec_s[iec] += 1;
            if vec_s[iec] == vec_t[iec] {
                good += 1;
            }
            end += 1;
            if good == 52 {
                loop {
                    let isc = Self::to_idx(s[start]);
                    if vec_t[isc] < vec_s[isc] {
                        start += 1;
                        vec_s[isc] -= 1;
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
