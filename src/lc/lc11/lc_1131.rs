// https://leetcode.com/problems/maximum-of-absolute-value-expression/
// 1131. Maximum of Absolute Value Expression
pub struct Solution;
impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut maxaa = i32::MIN;
        let mut maxas = i32::MIN;
        let mut maxsa = i32::MIN;
        let mut maxss = i32::MIN;
        let mut minaa = i32::MAX;
        let mut minas = i32::MAX;
        let mut minsa = i32::MAX;
        let mut minss = i32::MAX;
        for i in 0..arr1.len() {
            let a1 = arr1[i];
            let a2 = arr2[i];
            let ii = i as i32;
            let aa = a1 + a2 + ii;
            let as_ = a1 + a2 - ii;
            let sa = a1 - a2 + ii;
            let ss = a1 - a2 - ii;
            maxaa = maxaa.max(aa);
            maxas = maxas.max(as_);
            maxsa = maxsa.max(sa);
            maxss = maxss.max(ss);
            minaa = minaa.min(aa);
            minas = minas.min(as_);
            minsa = minsa.min(sa);
            minss = minss.min(ss);
        }
        (maxaa - minaa).max(maxas - minas).max(maxsa - minsa).max(maxss - minss)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_abs_val_expr() {
        assert_eq!(Solution::max_abs_val_expr(vec![1, 2, 3, 4], vec![-1, 4, 5, 6]), 13);
        assert_eq!(
            Solution::max_abs_val_expr(vec![1, -2, -5, 0, 10], vec![0, -2, -1, -7, -4]),
            20
        );
    }
}
