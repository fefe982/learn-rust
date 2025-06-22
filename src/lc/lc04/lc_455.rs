// https://leetcode.com/problems/assign-cookies/
// 455. Assign Cookies
pub struct Solution;
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_unstable();
        s.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        let mut cnt = 0;
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                cnt += 1;
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_content_children() {
        assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
        assert_eq!(Solution::find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
    }
}
