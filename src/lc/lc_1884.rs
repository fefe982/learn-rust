// https://leetcode.com/problems/egg-drop-with-2-eggs-and-n-floors/
// 1884. Egg Drop With 2 Eggs and N Floors
pub struct Solution;
impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let mut k = (((1.0 + 8.0 * n as f64).sqrt() - 1.0) / 2.0) as i32;
        while (k * (k + 1) / 2) < n {
            k += 1;
        }
        k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_egg_drop() {
        assert_eq!(Solution::two_egg_drop(2), 2);
        assert_eq!(Solution::two_egg_drop(100), 14);
    }
}
