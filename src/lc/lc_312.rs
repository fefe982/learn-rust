// https://leetcode.com/problems/burst-balloons/description/
// 312. Burst Balloons
pub struct Solution;
impl Solution {
    fn max_c(nums: &Vec<i32>, s: usize, e: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if cache[s][e - s] >= 0 {
            return cache[s][e - s];
        }
        let mut max = 0;
        for i in s..e {
            let mut sum = nums[i] * nums[s - 1] * nums[e];
            if s < i {
                sum += Self::max_c(nums, s, i, cache);
            }
            if i + 1 < e {
                sum += Self::max_c(nums, i + 1, e, cache);
            }
            max = std::cmp::max(sum, max);
        }
        cache[s][e - s] = max;
        max
    }
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        nums.insert(0, 1);
        nums.push(1);
        Self::max_c(
            &nums,
            1,
            nums.len() - 1,
            &mut vec![vec![-1; nums.len() - 1]; nums.len() - 1],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_coins() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
        assert_eq!(Solution::max_coins(vec![1, 5]), 10);
    }
}
