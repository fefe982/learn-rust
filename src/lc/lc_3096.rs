// https://leetcode.com/problems/minimum-levels-to-gain-more-points/
// 3096. Minimum Levels to Gain More Points
pub struct Solution;
impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let sum = possible.iter().map(|&x| if x == 1 { 1 } else { -1 }).sum::<i32>();
        let mut partial = 0;
        for i in 0..possible.len() - 1 {
            partial += if possible[i] == 1 { 1 } else { -1 };
            if partial > sum - partial {
                return i as i32 + 1;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_levels() {
        assert_eq!(Solution::minimum_levels(vec![1, 0, 1, 0]), 1);
        assert_eq!(Solution::minimum_levels(vec![1, 1, 1, 1, 1]), 3);
        assert_eq!(Solution::minimum_levels(vec![0, 0]), -1);
    }
}
