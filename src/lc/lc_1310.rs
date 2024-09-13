// https://leetcode.com/problems/xor-queries-of-a-subarray/
// 1310. XOR Queries of a Subarray
pub struct Solution;
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix = Vec::with_capacity(arr.len() + 1);
        prefix.push(0);
        for a in arr {
            prefix.push(prefix.last().unwrap() ^ a);
        }
        queries
            .into_iter()
            .map(|q| prefix[q[0] as usize] ^ prefix[q[1] as usize + 1])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_xor_queries() {
        assert_eq!(
            Solution::xor_queries(vec![1, 3, 4, 8], vec_vec![[0, 1], [1, 2], [0, 3], [3, 3]]),
            vec![2, 7, 14, 8]
        );
        assert_eq!(
            Solution::xor_queries(vec![4, 8, 2, 10], vec_vec![[2, 3], [1, 3], [0, 0], [0, 3]]),
            vec![8, 0, 4, 4]
        );
    }
}
