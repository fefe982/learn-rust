// https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/
// 1625. Lexicographically Smallest String After Applying Operations
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            a = a % b;
            if a == 0 {
                return b;
            }
            b = b % a;
        }
        a
    }
    fn calc_min_this(ch: u8, idx: usize, a: i32, b: i32) -> u8 {
        if idx % 2 == 1 || b % 2 == 1 {
            (ch - b'0') % a as u8
        } else {
            ch - b'0'
        }
    }
    fn get_diff(s: &[u8], idx: usize, a: i32, b: i32) -> (u8, u8) {
        (
            s[idx % s.len()] - b'0' - Self::calc_min_this(s[idx % s.len()], idx, a, b),
            s[(idx + 1) % s.len()]
                - b'0'
                - Self::calc_min_this(s[(idx + 1) % s.len()], idx + 1, a, b),
        )
    }
    fn compare_idx(s: &[u8], idx0: usize, idx1: usize, a: i32, b: i32) -> usize {
        let (diff00, diff01) = Self::get_diff(s, idx0, a, b);
        let (diff10, diff11) = Self::get_diff(s, idx1, a, b);
        for i in 1..s.len() / 2 {
            let u80 = (s[(idx0 + i * 2) % s.len()] - b'0' + 10 - diff00) % 10 + b'0';
            let u81 = (s[(idx1 + i * 2) % s.len()] - b'0' + 10 - diff10) % 10 + b'0';
            if u80 != u81 {
                return if u80 < u81 { idx0 } else { idx1 };
            }
            let u80 = (s[(idx0 + i * 2 + 1) % s.len()] - b'0' + 10 - diff01) % 10 + b'0';
            let u81 = (s[(idx1 + i * 2 + 1) % s.len()] - b'0' + 10 - diff11) % 10 + b'0';
            if u80 != u81 {
                return if u80 < u81 { idx0 } else { idx1 };
            }
        }
        idx0
    }
    fn get_str(s: &[u8], idx: usize, a: i32, b: i32) -> String {
        let mut u8v: Vec<u8> = s.iter().map(|x| *x).collect();
        let (diff0, diff1) = Self::get_diff(s, idx, a, b);
        for i in 0..s.len() / 2 {
            u8v[i * 2] = (s[(idx + i * 2) % s.len()] - b'0' + 10 - diff0) % 10 + b'0';
            u8v[i * 2 + 1] = (s[(idx + i * 2 + 1) % s.len()] - b'0' + 10 - diff1) % 10 + b'0';
        }
        String::from_utf8(u8v).unwrap()
    }
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let s = s.as_bytes();
        let a = Self::gcd(10, a);
        let b = Self::gcd(s.len() as i32, b);
        let mut min_first = 10u8;
        for i in 0..s.len() / (b as usize) {
            let idx = i * (b as usize);
            let min_this = Self::calc_min_this(s[idx], idx, a, b);
            if min_this < min_first {
                min_first = min_this;
            }
        }
        let mut min_second = 10u8;
        let mut min_idx = 0usize;
        for i in 0..s.len() / (b as usize) {
            let idx = i * (b as usize);
            let min_this = Self::calc_min_this(s[idx], idx, a, b);
            if min_this == min_first {
                let min_next = Self::calc_min_this(s[(idx + 1) % s.len()], idx + 1, a, b);
                if min_next < min_second {
                    min_second = min_next;
                    min_idx = idx;
                } else if min_next == min_second {
                    min_idx = Self::compare_idx(s, min_idx, idx, a, b);
                }
            }
        }
        Self::get_str(s, min_idx, a, b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_lex_smallest_string() {
        assert_eq!(
            Solution::find_lex_smallest_string(String::from("5525"), 9, 2),
            String::from("2050")
        );
        assert_eq!(
            Solution::find_lex_smallest_string(String::from("74"), 5, 1),
            String::from("24")
        );
        assert_eq!(
            Solution::find_lex_smallest_string(String::from("0011"), 4, 2),
            String::from("0011")
        );
    }
}
