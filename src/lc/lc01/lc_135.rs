// https://leetcode.com/problems/candy/
// 135. Candy
pub struct Solution;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![0; ratings.len()];
        for idx in 0..ratings.len() {
            if (idx == 0 || ratings[idx] <= ratings[idx - 1])
                && (idx == ratings.len() - 1 || ratings[idx] <= ratings[idx + 1])
            {
                candies[idx] = 1;
            } else if idx > 0
                && candies[idx - 1] != 0
                && ratings[idx] > ratings[idx - 1]
                && (idx == ratings.len() - 1 || ratings[idx] <= ratings[idx + 1])
            {
                candies[idx] = candies[idx - 1] + 1;
            }
        }
        for idx in (0..ratings.len()).rev() {
            if candies[idx] != 0 {
                continue;
            }
            if idx < ratings.len() - 1 && candies[idx + 1] != 0 && ratings[idx] > ratings[idx + 1] {
                if idx == 0 || ratings[idx] <= ratings[idx - 1] {
                    candies[idx] = candies[idx + 1] + 1;
                } else if idx > 0 && ratings[idx] > ratings[idx - 1] {
                    candies[idx] = std::cmp::max(candies[idx - 1], candies[idx + 1]) + 1;
                }
            }
        }
        let mut sum = 0;
        for c in candies {
            sum += c;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn candy() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }
}
