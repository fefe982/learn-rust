// https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/
// 1996. The Number of Weak Characters in the Game
pub struct Solution;
impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_unstable_by(|a, b| if a[0] == b[0] { b[1].cmp(&a[1]) } else { a[0].cmp(&b[0]) });
        let mut max_def = 0;
        let mut cnt = 0;
        for prop in properties.into_iter().rev() {
            if prop[1] < max_def {
                cnt += 1;
            } else {
                max_def = max_def.max(prop[1]);
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn number_of_weak_characters() {
        assert_eq!(
            Solution::number_of_weak_characters(vec_vec![[1, 1], [2, 1], [2, 2], [1, 2]]),
            1
        );
        assert_eq!(Solution::number_of_weak_characters(vec_vec![[5, 5], [6, 3], [3, 6]]), 0);
        assert_eq!(Solution::number_of_weak_characters(vec_vec![[2, 2], [3, 3]]), 1);
        assert_eq!(
            Solution::number_of_weak_characters(vec_vec![[1, 5], [10, 4], [4, 3]]),
            1
        );
    }
}
