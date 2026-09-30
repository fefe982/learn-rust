// https://leetcode.com/problems/find-the-number-of-copy-arrays/
// 3468. Find the Number of Copy Arrays
pub struct Solution;
impl Solution {
    pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        let mut low = bounds[0][0];
        let mut high = bounds[0][1];
        for i in 1..bounds.len() {
            low = (low + original[i] - original[i - 1]).max(bounds[i][0]);
            high = (high + original[i] - original[i - 1]).min(bounds[i][1]);
            if low > high {
                return 0;
            }
        }
        high - low + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_arrays() {
        assert_eq!(
            Solution::count_arrays(vec![1, 2, 3, 4], vec_vec![[1, 2], [2, 3], [3, 4], [4, 5]]),
            2
        );
        assert_eq!(
            Solution::count_arrays(vec![1, 2, 3, 4], vec_vec![[1, 10], [2, 9], [3, 8], [4, 7]]),
            4
        );
        assert_eq!(
            Solution::count_arrays(vec![1, 2, 1, 2], vec_vec![[1, 1], [2, 3], [3, 3], [2, 3]]),
            0
        );
    }
}
