// https://leetcode.com/problems/4sum/
// 18. 4Sum
pub struct Solution;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = vec![];
        if nums.len() < 4 {
            return res;
        }
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let t = target as i64 - nums[i] as i64 - nums[j] as i64;
                let mut l = j + 1;
                let mut r = l + 1;
                while r + 1 < nums.len() && (nums[l] as i64 + nums[r] as i64) < t {
                    r += 1;
                }
                while l < r {
                    if (nums[l] as i64 + nums[r] as i64) == t {
                        res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        while l < r && (nums[l] as i64 + nums[r] as i64) == t {
                            r = r - 1;
                        }
                    }
                    while l < r && (nums[l] as i64 + nums[r] as i64) > t {
                        r = r - 1;
                    }
                    while l < r && (nums[l] as i64 + nums[r] as i64) < t {
                        l = l + 1;
                    }
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_valid() {
        assert_eq!(
            Solution::four_sum(vec![1000000000, 1000000000, 1000000000, 1000000000], -294967296),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::four_sum(vec![-3, -1, 0, 2, 4, 5], 2),
            vec_vec![[-3, -1, 2, 4]]
        );
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec_vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
        assert_eq!(Solution::four_sum(vec![2, 2, 2, 2, 2], 8), vec_vec![[2, 2, 2, 2]]);
    }
}
