// https://leetcode.cn/problems/equal-sum-grid-partition-i/
// 3546. Equal Sum Grid Partition I
pub struct Solution;
impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        let mut sum = 0;
        let mut row_sum = vec![0; n];
        let mut col_sum = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                let val = grid[i][j] as i64;
                sum += val;
                row_sum[i] += val;
                col_sum[j] += val;
            }
        }
        if sum % 2 != 0 {
            return false;
        }
        let half = sum / 2;
        let mut hsum = 0;
        for i in 0..n {
            hsum += row_sum[i];
            if hsum == half {
                return true;
            } else if hsum > half {
                break;
            }
        }
        hsum = 0;
        for j in 0..m {
            hsum += col_sum[j];
            if hsum == half {
                return true;
            } else if hsum > half {
                break;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn can_partition_grid() {
        assert_eq!(Solution::can_partition_grid(vec_vec![[1, 4], [2, 3]]), true);
        assert_eq!(Solution::can_partition_grid(vec_vec![[1, 3], [2, 4]]), false);
    }
}
