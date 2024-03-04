// https://leetcode.com/problems/bag-of-tokens/
// 948. Bag of Tokens
pub struct Solution;
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }
        let mut max = 0;
        let mut tokens = tokens;
        tokens.sort_unstable();
        let mut i = 0;
        let mut j = tokens.len() - 1;
        let mut score = 0;
        let mut power = power;
        while i <= j {
            if power >= tokens[i] {
                power -= tokens[i];
                score += 1;
                i += 1;
                max = max.max(score);
            } else if score > 0 {
                power += tokens[j];
                score -= 1;
                j -= 1;
            } else {
                break;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bag_of_tokens_score() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
        assert_eq!(Solution::bag_of_tokens_score(vec![200, 100], 150), 1);
        assert_eq!(Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
    }
}
