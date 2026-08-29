// https://leetcode.com/problems/maximum-points-after-enemy-battles/
// 3207. Maximum Points After Enemy Battles
pub struct Solution;
impl Solution {
    pub fn maximum_points(enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
        let mut min = i32::MAX;
        let mut sum = 0;
        for e in enemy_energies {
            sum += e as i64;
            min = min.min(e);
        }
        if current_energy < min {
            0
        } else {
            (sum - min as i64 + current_energy as i64) / min as i64
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_points() {
        assert_eq!(Solution::maximum_points(vec![3, 2, 2], 2), 3);
        assert_eq!(Solution::maximum_points(vec![2], 10), 5);
    }
}
