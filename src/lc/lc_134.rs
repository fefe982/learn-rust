// https://leetcode.com/problems/gas-station/
// 134. Gas Station
pub struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = 0;
        let mut idx = 0;
        for (i, (g, c)) in gas.into_iter().zip(cost).enumerate() {
            sum += g - c;
            if sum < min {
                min = sum;
                idx = i + 1;
            }
        }
        if sum < 0 {
            return -1;
        }
        idx as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    }
}
