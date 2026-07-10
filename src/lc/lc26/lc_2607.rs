// https://leetcode.com/problems/make-k-subarray-sums-equal/
// 2607. Make K-Subarray Sums Equal
pub struct Solution;
impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let mut nl = arr.len() as i32;
        let mut nk = k;
        while nl % nk != 0 {
            let nn = nl % nk;
            nl = nk;
            nk = nn;
        }
        let nk = nk as usize;
        let mut v = vec![0; arr.len() / nk];
        let mut res = 0;
        for i in 0..nk {
            for j in 0..v.len() {
                v[j] = arr[j * nk + i];
            }
            v.sort_unstable();
            let m = v[v.len() / 2];
            for j in 0..v.len() {
                res += (v[j] - m).abs() as i64;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_sub_k_sum_equal() {
        assert_eq!(Solution::make_sub_k_sum_equal(vec![1, 4, 1, 3], 2), 1);
        assert_eq!(Solution::make_sub_k_sum_equal(vec![2, 5, 5, 7], 3), 5);
    }
}
