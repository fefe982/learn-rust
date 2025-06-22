// https://leetcode.com/problems/reaching-points/
// 780. Reaching Points
pub struct Solution;
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let mut tx = tx;
        let mut ty = ty;
        loop {
            if tx < sx {
                return false;
            }
            if sx == tx && sy <= ty && sy % sx == ty % tx {
                return true;
            }
            ty = ty % tx;
            if ty < sy {
                return false;
            }
            if sy == ty && sx <= tx && sx % sy == tx % ty {
                return true;
            }
            tx = tx % ty;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reaching_points() {
        assert_eq!(Solution::reaching_points(3, 7, 3, 4), false);
        assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
        assert_eq!(Solution::reaching_points(1, 1, 2, 2), false);
        assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
    }
}
