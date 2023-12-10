// https://leetcode.com/problems/climbing-stairs/
// 70. Climbing Stairs
pub struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n as i32;
        }
        let mut a = 1;
        let mut b = 2;
        for _ in 3..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
