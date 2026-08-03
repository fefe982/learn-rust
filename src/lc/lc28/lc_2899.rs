// https://leetcode.com/problems/last-visited-integers/
// 2899. Last Visited Integers
pub struct Solution;
impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut nums = nums;
        let mut cnt = 0;
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] == -1 {
                cnt += 1;
                if cnt > j {
                    ret.push(-1);
                } else {
                    ret.push(nums[j - cnt]);
                }
            } else {
                nums[j] = nums[i];
                j += 1;
                cnt = 0;
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_visited_integers() {
        assert_eq!(Solution::last_visited_integers(vec![1, 2, -1, -1, -1]), vec![2, 1, -1]);
        assert_eq!(Solution::last_visited_integers(vec![1, -1, 2, -1, -1]), vec![1, 2, 1]);
    }
}
