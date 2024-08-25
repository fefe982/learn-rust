// https://leetcode.com/problems/count-anagrams/
// 2514. Count Anagrams
pub struct Solution;
impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        let mut res = 1;
        let mut n = 0;
        let m = 1_0000_0000_7;
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let c = m / b;
                b = b * (c + 1) % m;
                a = a * (c + 1) % m;
            }
            a / b
        };
        for c in s.chars() {
            if c == ' ' {
                n = 0;
                cnt.fill(0);
                continue;
            }
            n += 1;
            res = res * n % m;
            cnt[(c as u8 - b'a') as usize] += 1;
            res = div(res, cnt[(c as u8 - b'a') as usize]);
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_anagrams() {
        assert_eq!(Solution::count_anagrams("too hot".to_string()), 18);
        assert_eq!(Solution::count_anagrams("aa".to_string()), 1);
    }
}
