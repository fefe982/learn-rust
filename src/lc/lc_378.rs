// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
// 378. Kth Smallest Element in a Sorted Matrix
pub struct Solution;
impl Solution {
    fn count(matrix: &Vec<Vec<i32>>, v: i32) -> i32 {
        let mut ans = 0;
        let mut i = matrix[0].len();
        for j in 0..matrix.len() {
            while i > 0 && matrix[j][i - 1] > v {
                i -= 1;
            }
            ans += i as i32;
        }
        ans
    }
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut low = matrix[0][0];
        if Self::count(&matrix, low) >= k {
            return low;
        }
        let mut high = matrix[n - 1][n - 1];
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::count(&matrix, mid) >= k {
                high = mid
            } else {
                low = mid
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_kth_smallest() {
        assert_eq!(
            Solution::kth_smallest(vec_vec![[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8),
            13
        );
        assert_eq!(Solution::kth_smallest(vec_vec![[-5]], 1), -5);
    }
}
