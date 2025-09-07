// https://leetcode.com/problems/minimum-absolute-difference/
// 1200. Minimum Absolute Difference
pub struct Solution;
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort();
        let mut diff = i32::MAX;
        let mut r = vec![];
        for i in 1..arr.len() {
            let d = arr[i] - arr[i - 1];
            if d < diff {
                r = vec![vec![arr[i - 1], arr[i]]];
                diff = d;
            } else if d == diff {
                r.push(vec![arr[i - 1], arr[i]]);
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_abs_difference() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
            vec_vec![[1, 2], [2, 3], [3, 4]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
            vec_vec![[1, 3]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            vec_vec![[-14, -10], [19, 23], [23, 27]]
        );
    }
}
