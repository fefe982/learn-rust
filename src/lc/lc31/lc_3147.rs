// https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon/
// 3147. Taking Maximum Energy from the Mystic Dungeon
pub struct Solution;
impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut sum;
        let mut max = i32::MIN;
        let k = k as usize;
        for i in 0..k {
            sum = 0;
            for j in energy.iter().skip(i).step_by(k) {
                sum = sum.max(0) + j;
            }
            max = max.max(sum);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_energy() {
        assert_eq!(Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3), 3);
        assert_eq!(Solution::maximum_energy(vec![-2, -3, -1], 2), -1);
    }
}
