// https://leetcode.com/problems/recover-the-original-array/
// 2122. Recover the Original Array
pub struct Solution;
impl Solution {
    pub fn recover_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        for i in 1..=nums.len() / 2 {
            let k = nums[i] - nums[0];
            if k == 0 || k % 2 == 1 {
                continue;
            }
            let mut q = std::collections::VecDeque::new();
            let mut ans = vec![];
            for j in 0..nums.len() {
                if let Some(&front) = q.front() {
                    if front == nums[j] {
                        q.pop_front();
                        continue;
                    }
                    if front < nums[j] {
                        break;
                    }
                }
                ans.push(nums[j] + k / 2);
                q.push_back(nums[j] + k);
            }
            if q.is_empty() {
                return ans;
            }
        }
        unreachable!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    fn check(mut nums: Vec<i32>) {
        let mut ans = Solution::recover_array(nums.clone());
        nums.sort_unstable();
        ans.sort_unstable();
        let k = ans[0] - nums[0];
        assert!(k > 0);
        let ans = ans
            .into_iter()
            .map(|x| [x + k, x - k])
            .flatten()
            .sorted()
            .collect::<Vec<_>>();
        assert_eq!(nums, ans);
    }
    #[test]
    fn test_recover_array() {
        check(vec![2, 10, 6, 4, 8, 12]);
        check(vec![1, 1, 3, 3]);
        check(vec![5, 435]);
    }
}
