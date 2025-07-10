// https://leetcode.com/problems/find-the-maximum-length-of-a-good-subsequence-ii/
// 3177. Find the Maximum Length of a Good Subsequence II
pub struct Solution;
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut m = std::collections::HashMap::new();
        let k = k as usize;
        let mut max = vec![0; k + 2];
        for n in nums {
            let len = m.entry(n).or_insert(vec![0; k + 1]);
            for j in (0..=k).rev() {
                len[j] = len[j].max(max[j]) + 1;
                max[j + 1] = max[j + 1].max(len[j]);
            }
        }
        max[max.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_length() {
        assert_eq!(Solution::maximum_length(vec![59, 60, 58, 59, 58, 58], 2), 5);
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
    }
}
