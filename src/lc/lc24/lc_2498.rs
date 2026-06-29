// https://leetcode.com/problems/frog-jump-ii/
// 2498. Frog Jump II
pub struct Solution;
impl Solution {
    pub fn max_jump(stones: Vec<i32>) -> i32 {
        if stones.len() == 2 {
            return stones[1] - stones[0];
        }
        let mut max_jump = 0;
        for i in 2..stones.len() {
            max_jump = max_jump.max(stones[i] - stones[i - 2]);
        }
        max_jump
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_jump() {
        assert_eq!(Solution::max_jump(vec![0, 2, 5, 6, 7]), 5);
        assert_eq!(Solution::max_jump(vec![0, 3, 9]), 9);
    }
}
