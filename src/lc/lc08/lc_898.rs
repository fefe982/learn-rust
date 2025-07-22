// https://leetcode.com/problems/bitwise-ors-of-subarrays/
// 898. Bitwise ORs of Subarrays
pub struct Solution;
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut s = HashSet::new();
        for i in 0..arr.len() {
            let mut ns = HashSet::new();
            for k in s {
                ns.insert(k | arr[i]);
                set.insert(k | arr[i]);
            }
            ns.insert(arr[i]);
            set.insert(arr[i]);
            s = ns;
        }
        set.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subarray_bitwise_o_rs() {
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![0]), 1);
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
    }
}
