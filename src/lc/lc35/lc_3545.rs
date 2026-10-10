// https://leetcode.com/problems/minimum-deletions-for-at-most-k-distinct-characters/
// 3545. Minimum Deletions for At Most K Distinct Characters
pub struct Solution;
impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut c = vec![0; 26];
        for cc in s.as_bytes() {
            c[(*cc - b'a') as usize] += 1;
        }
        c.sort_unstable();
        c.iter().take(26 - k as usize).sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_deletion() {
        assert_eq!(Solution::min_deletion("abc".to_string(), 2), 1);
        assert_eq!(Solution::min_deletion("aabb".to_string(), 2), 0);
        assert_eq!(Solution::min_deletion("yyyzz".to_string(), 1), 2);
    }
}
