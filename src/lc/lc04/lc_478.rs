// https://leetcode.com/problems/generate-random-point-in-a-circle/
// 478. Generate Random Point in a Circle
use rand;
pub struct Solution {
    x_center: f64,
    y_center: f64,
    radius: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            x_center,
            y_center,
            radius,
        }
    }

    pub fn rand_point(&self) -> Vec<f64> {
        let r = rand::random::<f64>().sqrt() * self.radius;
        let theta = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        vec![self.x_center + r * theta.cos(), self.y_center + r * theta.sin()]
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_478() {
        let obj = Solution::new(1.0, 0.0, 0.0);
        for _ in 0..3 {
            let ret_1: Vec<f64> = obj.rand_point();
            assert!(ret_1[0].powi(2) + ret_1[1].powi(2) < 1.0);
        }
    }
}
