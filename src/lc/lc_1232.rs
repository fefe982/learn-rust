// https://leetcode.com/problems/check-if-it-is-a-straight-line/
// 1232. Check If It Is a Straight Line
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            a = a % b;
            if a == 0 {
                return b;
            }
            b = b % a;
            if b == 0 {
                return a;
            }
        }
    }
    fn get_slope(x1: i32, y1: i32, x2: i32, y2: i32) -> (i32, i32) {
        let a = x2 - x1;
        let b = y2 - y1;
        if a == 0 {
            (0, 1)
        } else if b == 0 {
            (1, 0)
        } else {
            let c = Self::gcd(a.abs(), b.abs()) * if a < 0 { -1 } else { 1 };
            (a / c, b / c)
        }
    }
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let slope = Self::get_slope(
            coordinates[0][0],
            coordinates[0][1],
            coordinates[1][0],
            coordinates[1][1],
        );
        for i in 2..coordinates.len() {
            if Self::get_slope(
                coordinates[0][0],
                coordinates[0][1],
                coordinates[i][0],
                coordinates[i][1],
            ) != slope
            {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn check_straight_line() {
        assert_eq!(
            Solution::check_straight_line(vec_vec![[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7]]),
            true
        );
        assert_eq!(
            Solution::check_straight_line(vec_vec![[1, 1], [2, 2], [3, 4], [4, 5], [5, 6], [7, 7]]),
            false
        );
    }
}
