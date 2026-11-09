// https://leetcode.com/problems/climbing-stairs-ii/
// 3693. Climbing Stairs II
pub struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let mut costs = costs;
        let n = n as usize;
        for i in 0..n {
            let mut m = i32::MAX;
            for j in 1..=3 {
                if i >= j {
                    m = m.min(costs[i - j] + costs[i] + j as i32 * j as i32);
                }
            }
            if i <= 2 {
                m = m.min(costs[i] + (i + 1) as i32 * (i + 1) as i32);
            }
            costs[i] = m;
        }
        costs[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn climb_stairs() {
        assert_eq!(Solution::climb_stairs(4, vec![1, 2, 3, 4]), 13);
        assert_eq!(Solution::climb_stairs(4, vec![5, 1, 6, 2]), 11);
        assert_eq!(Solution::climb_stairs(3, vec![9, 8, 3]), 12);
    }
}
