// https://leetcode.com/problems/where-will-the-ball-fall/
// 1706. Where Will the Ball Fall
pub struct Solution;
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![-1; grid[0].len()];
        let w = grid[0].len();
        let h = grid.len();
        'out: for i in 0..w {
            let mut col = i;
            for j in 0..h {
                if col + 1 < w && grid[j][col] == 1 && grid[j][col + 1] == 1 {
                    col += 1;
                } else if col > 0 && grid[j][col] == -1 && grid[j][col - 1] == -1 {
                    col -= 1;
                } else {
                    continue 'out;
                }
            }
            res[i] = col as i32;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_ball() {
        assert_eq!(
            Solution::find_ball(vec_vec![
                [1, 1, 1, -1, -1],
                [1, 1, 1, -1, -1],
                [-1, -1, -1, 1, 1],
                [1, 1, 1, 1, -1],
                [-1, -1, -1, -1, -1]
            ]),
            vec![1, -1, -1, -1, -1]
        );
        assert_eq!(Solution::find_ball(vec_vec![[-1]]), vec![-1]);
    }
}
