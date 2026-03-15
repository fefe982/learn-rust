// https://leetcode.cn/problems/get-biggest-three-rhombus-sums-in-a-grid/
// 1878. Get Biggest Three Rhombus Sums in a Grid
pub struct Solution;
impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![-1, -2, -3];
        let n = grid.len();
        let m = grid[0].len();
        let mut sum1 = vec![vec![0; m + 2]; n + 1];
        let mut sum2 = vec![vec![0; m + 2]; n + 1];
        for i in 0..n {
            for j in 0..m {
                sum1[i + 1][j + 1] = sum1[i][j] + grid[i][j];
                sum2[i + 1][j + 1] = sum2[i][j + 2] + grid[i][j];
            }
        }
        let mut add = |x: i32| {
            if x > ans[0] {
                ans[2] = ans[1];
                ans[1] = ans[0];
                ans[0] = x;
            } else if x == ans[0] {
                return;
            } else if x > ans[1] {
                ans[2] = ans[1];
                ans[1] = x;
            } else if x == ans[1] {
                return;
            } else if x > ans[2] {
                ans[2] = x;
            }
        };
        for i in 0..n {
            for j in 0..m {
                let mut li = i;
                let mut lj = j;
                let mut ri = i;
                let mut rj = j;
                let mut di = i;
                let dj = j;
                add(grid[i][j]);
                loop {
                    if lj > 0 && rj + 1 < m && di + 2 < n {
                        li += 1;
                        lj -= 1;
                        ri += 1;
                        rj += 1;
                        di += 2;
                    } else {
                        break;
                    }
                    let s = sum2[li + 1][lj + 1] - sum2[i][j + 2] + sum1[ri + 1][rj + 1] - sum1[i][j]
                        + sum1[di + 1][dj + 1]
                        - sum1[li][lj]
                        + sum2[di + 1][dj + 1]
                        - sum2[ri][rj + 2]
                        - grid[i][j]
                        - grid[li][lj]
                        - grid[ri][rj]
                        - grid[di][dj];
                    add(s);
                }
            }
        }
        while let Some(&x) = ans.last() {
            if x < 0 {
                ans.pop();
            } else {
                break;
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
    fn get_biggest_three() {
        assert_eq!(
            Solution::get_biggest_three(vec_vec![
                [3, 4, 5, 1, 3],
                [3, 3, 4, 2, 3],
                [20, 30, 200, 40, 10],
                [1, 5, 5, 4, 1],
                [4, 3, 2, 2, 5]
            ]),
            [228, 216, 211]
        );
        assert_eq!(
            Solution::get_biggest_three(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            [20, 9, 8]
        );
        assert_eq!(Solution::get_biggest_three(vec_vec![[7, 7, 7]]), [7]);
    }
}
