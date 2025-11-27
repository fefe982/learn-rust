// https://leetcode.com/problems/string-transformation/
// 2851. String Transformation
pub struct Solution;
impl Solution {
    pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = s.len();
        let m = 1_0000_0000_7;
        let pow = |a: i64, b: i64| -> i64 {
            let mut p = a;
            let mut bit = 1;
            let mut ans = 1;
            while bit <= b {
                if bit & b != 0 {
                    ans = (ans * p) % m;
                }
                p = (p * p) % m;
                bit <<= 1;
            }
            ans
        };
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let nn = m / b;
                a = (a * (nn + 1)) % m;
                b = (b * (nn + 1)) % m;
            }
            a / b
        };
        let m1k = if k % 2 == 0 { 1 } else { -1 };
        let fi = div((pow(n as i64 - 1, k) - m1k + m) % m, n as i64);
        let f0 = fi + m1k;
        let getnext = || -> Vec<usize> {
            let mut next = vec![0; n];
            let mut j = 0;
            for i in 1..n {
                while j > 0 && t[j] != t[i] {
                    j = next[j - 1];
                }
                if t[j] == t[i] {
                    j += 1;
                }
                next[i] = j;
            }
            next
        };
        let next = getnext();
        let mut ans = 0;
        let mut j = 0;
        for i in 0..n * 2 - 1 {
            while j > 0 && s[i % n] != t[j] {
                j = next[j - 1];
            }
            if s[i % n] == t[j] {
                j += 1;
            }
            if j == n {
                ans = (ans + if i == n - 1 { f0 } else { fi }) % m;
                j = next[j - 1];
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_ways() {
        assert_eq!(Solution::number_of_ways("ababa".to_string(), "abaab".to_string(), 1), 1);
        assert_eq!(
            Solution::number_of_ways("nnjqjmgome".to_string(), "gbfuecwlqc".to_string(), 359221508193514),
            0
        );
        assert_eq!(Solution::number_of_ways("abcd".to_string(), "cdab".to_string(), 2), 2);
        assert_eq!(
            Solution::number_of_ways("ababab".to_string(), "ababab".to_string(), 1),
            2
        );
    }
}
