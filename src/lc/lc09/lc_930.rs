// https://leetcode.com/problems/binary-subarrays-with-sum/
// 930. Binary Subarrays With Sum
pub struct Solution;
impl Solution {
    fn count(pos: &std::collections::VecDeque<i32>) -> i32 {
        if pos.len() == 2 {
            let d = pos[1] - pos[0];
            d * (d - 1) / 2
        } else {
            let n = pos.len();
            (pos[1] - pos[0]) * (pos[n - 1] - pos[n - 2])
        }
    }
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut ans = 0;
        let mut pos = std::collections::VecDeque::<i32>::new();
        pos.push_back(-1);
        let goal = goal as usize;
        let len = nums.len();
        for (i, n) in nums.into_iter().enumerate() {
            if n == 1 {
                pos.push_back(i as i32);
                if pos.len() == goal + 2 {
                    ans += Self::count(&pos);
                    pos.pop_front();
                }
            }
        }
        pos.push_back(len as i32);
        if pos.len() == goal + 2 {
            ans += Self::count(&pos);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_subarrays_with_sum() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }
}
