// https://leetcode.cn/problems/queens-that-can-attack-the-king/
// 1222. Queens That Can Attack the King
pub struct Solution;
impl Solution {
    fn dist(q: &Vec<i32>, k: &Vec<i32>) -> Option<(usize, i32)> {
        let (mut dx, mut dy) = (k[0] - q[0], k[1] - q[1]);
        if dx == 0 && dy == 0 {
            None
        } else {
            let mut d = 0;
            if dx == 0 {
                d = dy.abs();
                dy /= d;
            } else if dy == 0 {
                d = dx.abs();
                dx /= d;
            } else if dx.abs() == dy.abs() {
                d = dx.abs();
                dx /= d;
                dy /= d;
            }
            if d == 0 {
                None
            } else {
                Some((((dx + 1) * 3 + dy + 1) as usize, d))
            }
        }
    }
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![(i32::MAX, -1, -1); 9];
        for queen in queens {
            if let Some((i, d)) = Self::dist(&queen, &king) {
                if d < res[i].0 {
                    res[i] = (d, queen[0], queen[1]);
                }
            }
        }
        res.into_iter()
            .filter_map(|(d, q0, q1)| {
                if d == i32::MAX {
                    None
                } else {
                    Some(vec![q0, q1])
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_queens_attackthe_king() {
        assert_eq!(
            Solution::queens_attackthe_king(
                vec_vec![[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]],
                vec![0, 0]
            ),
            vec_vec![[3, 3], [1, 0], [0, 1]]
        );
        assert_eq!(
            Solution::queens_attackthe_king(
                vec_vec![[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]],
                vec![3, 3]
            ),
            vec_vec![[4, 4], [3, 4], [2, 2]]
        );
    }
}
