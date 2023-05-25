// https://leetcode.com/problems/new-21-game/description/
// 837. New 21 Game
pub struct Solution;
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 || n > max_pts + k - 1 {
            return 1.0;
        }
        let mut p = vec![0.0; k as usize];
        p[0] = (n - k + 1) as f64 / max_pts as f64;
        if k == 1 {
            return p[0];
        }
        for i in 2..=max_pts as usize + k as usize - n as usize {
            p[i - 1] = p[i - 2] * (1.0 + 1.0 / max_pts as f64);
            if k as usize == i {
                return p[i - 1];
            }
        }
        for i in max_pts as usize + k as usize - n as usize + 1..=max_pts as usize + 1 {
            p[i - 1] = p[i - 2] * (1.0 + 1.0 / max_pts as f64) - 1.0 / max_pts as f64;
            if k as usize == i {
                return p[i - 1];
            }
        }
        for i in max_pts as usize + 2..=k as usize {
            p[i - 1] = p[i - 2] * (1.0 + 1.0 / max_pts as f64)
                - p[i - 2 - max_pts as usize] / max_pts as f64;
        }
        p[k as usize - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn new21_game() {
        assert_approx_eq!(Solution::new21_game(10, 1, 10), 1.0);
        assert_approx_eq!(Solution::new21_game(6, 1, 10), 0.6);
        assert_approx_eq!(Solution::new21_game(21, 17, 10), 0.73278, 1e-5);
    }
}
