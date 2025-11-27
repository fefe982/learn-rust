// https://leetcode.com/problems/minimum-moves-to-spread-stones-over-grid/
// 2850. Minimum Moves to Spread Stones Over Grid
pub struct Solution;
impl Solution {
    fn permute(nums: &mut Vec<usize>) -> bool {
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                for j in 0..i {
                    if nums[j] < nums[i] {
                        nums.swap(i, j);
                        for k in 0..i / 2 {
                            nums.swap(k, i - k - 1);
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut src = vec![];
        let mut dst = vec![];
        let mut idx = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 1 {
                    idx.resize(idx.len() + grid[i][j] as usize - 1, src.len());
                    src.push((i, j));
                } else if grid[i][j] == 0 {
                    dst.push((i, j));
                }
            }
        }
        if idx.is_empty() {
            return 0;
        }
        let mut min = i32::MAX;
        loop {
            let mut sum = 0;
            for (&isrc, dst) in idx.iter().zip(dst.iter()) {
                sum += (src[isrc].0 as i32 - dst.0 as i32).abs() + (src[isrc].1 as i32 - dst.1 as i32).abs();
            }
            min = min.min(sum);
            if !Solution::permute(&mut idx) {
                return min;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_moves() {
        assert_eq!(Solution::minimum_moves(vec_vec![[1, 1, 0], [1, 1, 1], [1, 2, 1]]), 3);
        assert_eq!(Solution::minimum_moves(vec_vec![[1, 3, 0], [1, 0, 0], [1, 0, 3]]), 4);
    }
}
