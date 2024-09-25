// https://leetcode.com/problems/sum-of-prefix-scores-of-strings/
// 2416. Sum of Prefix Scores of Strings
pub struct Solution;
impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut words = words.into_iter().enumerate().collect::<Vec<_>>();
        words.sort_by(|a, b| a.1.cmp(&b.1));
        let mut common_prefix_length = vec![0; words.len()];
        for i in 1..words.len() {
            common_prefix_length[i] = words[i - 1]
                .1
                .chars()
                .zip(words[i].1.chars())
                .take_while(|(a, b)| a == b)
                .count() as i32;
        }
        let mut ans = vec![0; words.len()];
        for (i, (idx, word)) in words.iter().enumerate() {
            let mut prefix_len = word.len() as i32;
            ans[*idx] += prefix_len;
            for j in i + 1..words.len() {
                prefix_len = prefix_len.min(common_prefix_length[j]);
                if prefix_len == 0 {
                    break;
                }
                ans[*idx] += prefix_len;
                ans[words[j].0] += prefix_len;
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
    fn test_sum_prefix_scores() {
        assert_eq!(
            Solution::sum_prefix_scores(vec_str!["abc", "ab", "bc", "b"]),
            [5, 4, 3, 2]
        );
        assert_eq!(Solution::sum_prefix_scores(vec_str!["abcd"]), [4]);
    }
}
