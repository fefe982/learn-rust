// https://leetcode.com/problems/lexicographically-smallest-string-after-adjacent-removals/
// 3563. Lexicographically Smallest String After Adjacent Removals
pub struct Solution;
impl Solution {
    pub fn lexicographically_smallest_string(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut e = vec![vec![false; n + 1]; n + 1];
        for i in (0..n).rev() {
            e[i][i] = true;
            for j in 1..=(n - i) / 2 {
                if e[i + 1][i + j * 2 - 1] {
                    let d = (s[i] as i32 - s[i + j * 2 - 1] as i32).abs();
                    if d == 1 || d == 25 {
                        e[i][i + j * 2] = true;
                        continue;
                    }
                }
                for k in 1..j {
                    if e[i][i + k * 2] && e[i + k * 2][i + j * 2] {
                        e[i][i + j * 2] = true;
                        break;
                    }
                }
            }
        }
        let mut sub = vec!["".to_string(); n + 1];
        for i in (0..n).rev() {
            let mut r = s[i].to_string() + sub[i + 1].as_str();
            for j in 1..=(n - i) / 2 {
                if e[i][i + j * 2] {
                    r = r.min(sub[i + j * 2].clone());
                }
            }
            sub[i] = r;
        }
        sub[0].clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexicographically_smallest_string() {
        assert_eq!(Solution::lexicographically_smallest_string("abc".to_string()), "a");
        assert_eq!(Solution::lexicographically_smallest_string("bcda".to_string()), "");
        assert_eq!(Solution::lexicographically_smallest_string("zdce".to_string()), "zdce");
    }
}
