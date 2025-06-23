// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array
// 2200. Find All K-Distant Indices in an Array
pub struct Solution;
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut last = 0;
        let mut res = vec![];
        let len = nums.len() as i32;
        for (i, n) in nums.into_iter().enumerate() {
            if n == key {
                let low = last.max(i as i32 - k);
                let high = (i as i32 + k).min(len - 1);
                for j in low..=high {
                    res.push(j);
                }
                last = high + 1;
                if last >= len {
                    break;
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_k_distant_indices() {
        assert_eq!(
            Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            vec![0, 1, 2, 3, 4]
        );
    }
}
