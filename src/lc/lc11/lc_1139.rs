// https://leetcode.com/problems/largest-1-bordered-square/
// 1139. Largest 1-Bordered Square
pub struct Solution;
impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut right = vec![vec![0; n]; m];
        let mut down = vec![vec![0; n]; m];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if grid[i][j] == 1 {
                    right[i][j] = if j + 1 < n { right[i][j + 1] + 1 } else { 1 };
                    down[i][j] = if i + 1 < m { down[i + 1][j] + 1 } else { 1 };
                }
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                let len = right[i][j].min(down[i][j]);
                for k in (1..=len).rev() {
                    if right[i + k - 1][j] >= k && down[i][j + k - 1] >= k {
                        ans = ans.max(k);
                        break;
                    } else if k <= ans {
                        break;
                    }
                }
            }
        }
        (ans * ans) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest1_bordered_square() {
        assert_eq!(
            Solution::largest1_bordered_square(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            9
        );
        assert_eq!(Solution::largest1_bordered_square(vec![vec![1, 1, 0, 0]]), 1);
    }
}
