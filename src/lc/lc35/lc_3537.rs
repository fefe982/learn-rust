// https://leetcode.com/problems/fill-a-special-grid/
// 3537. Fill a Special Grid
pub struct Solution;
impl Solution {
    pub fn special_grid(n: i32) -> Vec<Vec<i32>> {
        if n == 0 {
            return vec![vec![0]];
        }
        let nn = (1 << n) as usize;
        let mut ans = vec![vec![0; nn]; nn];
        for i in 0..nn {
            for j in 0..nn {
                for k in (0..n).rev() {
                    let m = 1 << k;
                    match (i & m == 0, j & m == 0) {
                        (false, false) => {
                            ans[i][j] += 1 << (2 * k);
                        }
                        (false, true) => {
                            ans[i][j] += 2 << (2 * k);
                        }
                        (true, true) => {
                            ans[i][j] += 3 << (2 * k);
                        }
                        _ => {}
                    }
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn special_grid() {
        assert_eq!(Solution::special_grid(0), vec_vec![[0]]);
        assert_eq!(Solution::special_grid(1), vec_vec![[3, 0], [2, 1]]);
        assert_eq!(
            Solution::special_grid(2),
            vec_vec![[15, 12, 3, 0], [14, 13, 2, 1], [11, 8, 7, 4], [10, 9, 6, 5]]
        );
    }
}
