// https://leetcode.cn/problems/jump-game-ii/
// 45. Jump Game II
pub struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let mut c = 0;
        let mut cnt = 0;
        let mut n = 0;
        loop {
            cnt += 1;
            let mut nn = 0;
            for i in c..=n {
                nn = nn.max(nums[i] as usize + i);
                if nn + 1 >= nums.len() {
                    return cnt;
                }
            }
            c = n + 1;
            n = nn;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jump() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
