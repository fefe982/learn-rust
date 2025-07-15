// https://leetcode.com/problems/maximum-energy-boost-from-two-drinks/
// 3259. Maximum Energy Boost from Two Drinks
pub struct Solution;
impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let mut a = 0;
        let mut b = 0;
        let mut last_a = 0;
        let mut last_b = 0;
        for (ea, eb) in energy_drink_a.into_iter().zip(energy_drink_b.into_iter()) {
            let na = a.max(last_b) + ea as i64;
            let nb = b.max(last_a) + eb as i64;
            last_a = a;
            last_b = b;
            a = na;
            b = nb;
        }
        a.max(b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_energy_boost() {
        assert_eq!(Solution::max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]), 5);
        assert_eq!(Solution::max_energy_boost(vec![4, 1, 1], vec![1, 1, 3]), 7);
    }
}
