// https://leetcode.com/problems/escape-the-ghosts/
// 789. Escape The Ghosts
pub struct Solution;
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let d = target[0].abs() + target[1].abs();
        for g in ghosts {
            if (g[0] - target[0]).abs() + (g[1] - target[1]).abs() <= d {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn escape_ghosts() {
        assert_eq!(Solution::escape_ghosts(vec_vec![[1, 0], [0, 3]], vec![0, 1]), true);
        assert_eq!(Solution::escape_ghosts(vec_vec![[1, 0]], vec![2, 0]), false);
        assert_eq!(Solution::escape_ghosts(vec_vec![[2, 0]], vec![1, 0]), false);
    }
}
