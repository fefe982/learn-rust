// https://leetcode.com/problems/distinct-subsequences/
// 115. Distinct Subsequences
pub struct Solution;
impl Solution {
    fn cnt(s: &[u8], t: &[u8], src: &Vec<i32>, dst: &mut Vec<i32>, idx: usize) {
        for i in 0..(s.len() - t.len() + 1) {
            dst[i + 1] = dst[i];
            if s[i + idx] == t[idx] {
                dst[i + 1] += src[i + 1];
            }
        }
    }
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut v1 = vec![1; s.len() - t.len() + 2];
        v1[0] = 0;
        let mut v2 = vec![0; s.len() - t.len() + 2];
        for idx in 0..t.len() {
            if idx % 2 == 0 {
                Self::cnt(s, t, &v1, &mut v2, idx);
            } else {
                Self::cnt(s, t, &v2, &mut v1, idx);
            }
        }
        if t.len() % 2 == 1 {
            v2[s.len() - t.len() + 1]
        } else {
            v1[s.len() - t.len() + 1]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_distinct() {
        assert_eq!(
            Solution::num_distinct(String::from("rabbbit"), String::from("rabbit")),
            3
        );
        assert_eq!(
            Solution::num_distinct(String::from("babgbag"), String::from("bag")),
            5
        );
    }
}
