// https://leetcode.com/problems/all-divisions-with-the-highest-score-of-a-binary-array/
// 2155. All Divisions With the Highest Score of a Binary Array

pub struct Solution;
impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0];
        let mut max = 0;
        let mut sum = 0;
        for (idx, num) in (1..).zip(nums.iter()) {
            if *num == 0 {
                sum += 1;
            } else {
                sum -= 1;
            }
            if sum > max {
                max = sum;
                result = vec![idx];
            } else if sum == max {
                result.push(idx);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score_indices() {
        assert_eq!(Solution::max_score_indices(vec![0, 0, 1, 0]), vec![2, 4]);
        assert_eq!(Solution::max_score_indices(vec![0, 0, 0]), vec![3])
    }
}
