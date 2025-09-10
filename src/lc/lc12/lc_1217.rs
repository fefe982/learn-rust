// https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/
// 1217. Minimum Cost to Move Chips to The Same Position
pub struct Solution;
#[cfg(test)]
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd = 0;
        let mut even = 0;
        for i in position {
            if i % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        odd.min(even)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost_to_move_chips() {
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
        assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 1000000000]), 1);
    }
}
