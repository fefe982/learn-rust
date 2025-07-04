// https://leetcode.com/problems/positions-of-large-groups/
// 830. Positions of Large Groups
pub struct Solution;
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut last = '.';
        let mut si = 0;
        let mut res = vec![];
        for (i, c) in s.chars().chain(['.']).enumerate() {
            if c != last {
                if i - si >= 3 {
                    res.push(vec![si as i32, i as i32 - 1]);
                }
                si = i;
            }
            last = c;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn large_group_positions() {
        assert_eq!(
            Solution::large_group_positions("abbxxxxzzy".to_string()),
            vec_vec![[3, 6]]
        );
        assert_eq!(
            Solution::large_group_positions("abc".to_string()),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
            vec_vec![[3, 5], [6, 9], [12, 14]]
        );
    }
}
