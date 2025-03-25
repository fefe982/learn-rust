// https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections
// 3394. Check if Grid can be Cut into Squares
pub struct Solution;
impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut bond = rectangles.iter().map(|r| (r[0], r[2])).collect::<Vec<_>>();
        bond.sort_unstable();
        let mut cut = 0;
        let mut end = 0;
        for (l, r) in bond {
            if l >= end {
                cut += 1;
                if cut == 3 {
                    return true;
                }
            }
            end = end.max(r);
        }
        end = 0;
        cut = 0;
        bond = rectangles.iter().map(|r| (r[1], r[3])).collect::<Vec<_>>();
        bond.sort_unstable();
        for (l, r) in bond {
            if l >= end {
                cut += 1;
                if cut == 3 {
                    return true;
                }
            }
            end = end.max(r);
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_valid_cuts() {
        assert_eq!(
            Solution::check_valid_cuts(5, vec_vec![[1, 0, 5, 2], [0, 2, 2, 4], [3, 2, 5, 3], [0, 4, 4, 5]]),
            true
        );
        assert_eq!(
            Solution::check_valid_cuts(3, vec_vec![[0, 0, 1, 1], [2, 0, 3, 4], [0, 2, 2, 3], [3, 0, 4, 3]]),
            true
        );
        assert_eq!(
            Solution::check_valid_cuts(
                3,
                vec_vec![[0, 2, 2, 4], [1, 0, 3, 2], [2, 2, 3, 4], [3, 0, 4, 2], [3, 2, 4, 4]]
            ),
            false
        );
    }
}
