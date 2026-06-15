// https://leetcode.com/problems/maximum-number-of-pairs-in-array/
// 2341. Maximum Number of Pairs in Array
pub struct Solution;
impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt = [0; 101];
        for num in nums {
            cnt[num as usize] += 1;
        }
        let mut pairs = 0;
        let mut left = 0;
        for c in cnt {
            pairs += c / 2;
            left += c % 2;
        }
        vec![pairs, left]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_pairs() {
        assert_eq!(Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]), vec![3, 1]);
        assert_eq!(Solution::number_of_pairs(vec![1, 1]), vec![1, 0]);
    }
}
