// https://leetcode.com/problems/minimum-operations-to-make-the-array-k-increasing/
// 2111. Minimum Operations to Make the Array K-Increasing
pub struct Solution;
impl Solution {
    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut res = 0;
        for i in 0..k {
            let mut seq = vec![];
            let mut idx = i;
            let mut j = 0;
            loop {
                if idx >= arr.len() {
                    break;
                }
                let ins = seq.partition_point(|x| arr[idx] >= *x);
                if ins == seq.len() {
                    seq.push(arr[idx]);
                } else {
                    seq[ins] = arr[idx];
                }
                idx += k;
                j += 1;
            }
            res += (j - seq.len()) as i32;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_k_increasing() {
        assert_eq!(Solution::k_increasing(vec![5, 4, 3, 2, 1], 1), 4);
        assert_eq!(Solution::k_increasing(vec![4, 1, 5, 2, 6, 2], 2), 0);
    }
}
