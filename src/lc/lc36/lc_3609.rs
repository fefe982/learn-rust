// https://leetcode.com/problems/minimum-moves-to-reach-target-in-grid/
// 3609. Minimum Moves to Reach Target in a Grid
pub struct Solution;
impl Solution {
    pub fn min_moves(sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
        let mut tx = tx;
        let mut ty = ty;
        for steps in 0.. {
            if sx > tx || sy > ty {
                return -1;
            }
            if sx == tx && sy == ty {
                return steps;
            }
            match tx.cmp(&ty) {
                std::cmp::Ordering::Equal => {
                    if sx == 0 {
                        tx = 0;
                    } else if sy == 0 {
                        ty = 0;
                    } else {
                        return -1;
                    }
                }
                std::cmp::Ordering::Greater => {
                    if tx > ty * 2 {
                        if tx % 2 == 1 {
                            return -1;
                        }
                        tx /= 2;
                    } else {
                        tx -= ty;
                    }
                }
                std::cmp::Ordering::Less => {
                    if ty > tx * 2 {
                        if ty % 2 == 1 {
                            return -1;
                        }
                        ty /= 2;
                    } else {
                        ty -= tx;
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_moves() {
        assert_eq!(Solution::min_moves(1, 2, 5, 4), 2);
        assert_eq!(Solution::min_moves(0, 1, 2, 3), 3);
        assert_eq!(Solution::min_moves(1, 1, 2, 2), -1);
    }
}
