// https://leetcode.com/problems/count-stable-subarrays/
// 3748. Count Stable Subarrays
pub struct Solution;
impl Solution {
    pub fn count_stable_subarrays(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut right = vec![0; nums.len()];
        let mut sum = vec![0; nums.len() + 1];
        let mut cnt = 0;
        let mut l = 0;
        for i in 0..nums.len() {
            if i > 0 && nums[i] < nums[i - 1] {
                while l < i {
                    right[l] = i - 1;
                    l += 1;
                }
                cnt = 1;
            } else {
                cnt += 1;
            }
            sum[i + 1] = sum[i] + cnt;
        }
        while l < nums.len() {
            right[l] = nums.len() - 1;
            l += 1;
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            if right[l] >= r {
                ans.push((r - l + 1) as i64 * (r - l + 2) as i64 / 2);
            } else {
                ans.push(sum[r + 1] - sum[right[l] + 1] + (right[l] - l + 1) as i64 * (right[l] - l + 2) as i64 / 2);
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
    fn count_stable_subarrays() {
        assert_eq!(
            Solution::count_stable_subarrays(vec![3, 1, 2], vec_vec![[0, 1], [1, 2], [0, 2]]),
            vec![2, 3, 4]
        );
        assert_eq!(
            Solution::count_stable_subarrays(vec![2, 2], vec_vec![[0, 1], [0, 0]]),
            vec![3, 1]
        );
    }
}
