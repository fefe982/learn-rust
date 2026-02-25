// https://leetcode.com/problems/mean-of-array-after-removing-some-elements/
// 1619. Mean of Array After Removing Some Elements
pub struct Solution;
impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort();
        let len = arr.len();
        let sum = arr[len / 20..len * 19 / 20].iter().sum::<i32>();
        sum as f64 / (len * 9 / 10) as f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn trim_mean() {
        assert_approx_eq!(
            Solution::trim_mean(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3]),
            2.0
        );
        assert_approx_eq!(
            Solution::trim_mean(vec![6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0]),
            4.0
        );
        assert_approx_eq!(
            Solution::trim_mean(vec![
                6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10, 8, 6, 6, 1, 0,
                6, 10, 8, 2, 3, 4
            ]),
            4.77778,
            1e-5
        );
    }
}
