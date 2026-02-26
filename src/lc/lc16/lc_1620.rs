// https://leetcode.com/problems/coordinate-with-maximum-network-quality/
// 1620. Coordinate With Maximum Network Quality
pub struct Solution;
impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let mut max = 0;
        let mut res = vec![0, 0];
        for x in 0..=50 {
            for y in 0..=50 {
                let mut q = 0;
                for tower in &towers {
                    let dx = (x - tower[0]).abs();
                    let dy = (y - tower[1]).abs();
                    let dd = dx * dx + dy * dy;
                    if dd <= radius * radius {
                        q += (tower[2] as f64 / (1.0 + (dd as f64).sqrt())) as i32;
                    }
                }
                if q > max {
                    max = q;
                    res = vec![x, y];
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn best_coordinate() {
        assert_eq!(
            Solution::best_coordinate(vec_vec![[1, 2, 5], [2, 1, 7], [3, 1, 9]], 2),
            [2, 1]
        );
        assert_eq!(Solution::best_coordinate(vec_vec![[23, 11, 21]], 9), [23, 11]);
        assert_eq!(
            Solution::best_coordinate(vec_vec![[1, 2, 13], [2, 1, 7], [0, 1, 9]], 2),
            [1, 2]
        );
    }
}
