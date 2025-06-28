// https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
// 1423. Maximum Points You Can Obtain from Cards
pub struct Solution;
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let mut sum = card_points.iter().take(k as usize).sum::<i32>();
        let k = k as usize;
        let mut max_score = sum;
        for i in 0..k {
            sum = sum - card_points[k - i - 1] + card_points[card_points.len() - 1 - i];
            max_score = max_score.max(sum);
        }
        max_score
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_score() {
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
        assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
        assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    }
}
