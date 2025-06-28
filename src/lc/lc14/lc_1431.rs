// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
// 1431. Kids With the Greatest Number of Candies
pub struct Solution;
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max = 0;
        for &c in &candies {
            max = std::cmp::max(c, max);
        }
        candies.iter().map(|&c| extra_candies + c >= max).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kids_with_candies() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
        assert_eq!(
            Solution::kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }
}
