// https://leetcode.com/problems/minimum-moves-to-capture-the-queen/
// 3001. Minimum Moves to Capture the Queen
pub struct Solution;
impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        if a == e {
            if a == c && ((b < d && d < f) || (b > d && d > f)) {
                2
            } else {
                1
            }
        } else if b == f {
            if b == d && ((a < c && c < e) || (a > c && c > e)) {
                2
            } else {
                1
            }
        } else if c - d == e - f {
            if a - b == c - d && ((c < a && a < e) || (c > a && a > e)) {
                2
            } else {
                1
            }
        } else if (c + d) == (e + f) {
            if a + b == c + d && ((c < a && a < e) || (c > a && a > e)) {
                2
            } else {
                1
            }
        } else {
            2
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_moves_to_capture_the_queen() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(4, 3, 3, 4, 5, 2), 2);
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 3), 2);
        assert_eq!(Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 2), 1);
    }
}
