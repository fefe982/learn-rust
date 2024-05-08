// https://leetcode.com/problems/relative-ranks/
// 506. Relative Ranks
pub struct Solution;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score = score.into_iter().enumerate().collect::<Vec<_>>();
        score.sort_by_key(|x| std::cmp::Reverse(x.1));
        let mut ans = vec!["".to_string(); score.len()];
        for (i, (j, _)) in score.into_iter().enumerate() {
            ans[j] = match i {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (i + 1).to_string(),
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_relative_ranks() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec_str!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec_str!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        );
    }
}
