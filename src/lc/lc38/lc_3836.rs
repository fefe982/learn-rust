// https://leetcode.com/problems/maximum-score-using-exactly-k-pairs/
// 3836. Maximum Score Using Exactly K Pairs
pub struct Solution;
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let mut max = vec![vec![0; l2 + 1]; l1 + 1];
        for ik in 1..=k as usize {
            let mut imax = vec![vec![i64::MIN; l2 + 1]; l1 + 1];
            for i in ik..=l1 {
                for j in ik..=l2 {
                    imax[i][j] = imax[i - 1][j]
                        .max(imax[i][j - 1])
                        .max(max[i - 1][j - 1] + nums1[i - 1] as i64 * nums2[j - 1] as i64);
                }
            }
            max = imax;
        }
        max[l1][l2]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score() {
        assert_eq!(Solution::max_score(vec![1, 3, 2], vec![4, 5, 1], 2), 22);
        assert_eq!(Solution::max_score(vec![-2, 0, 5], vec![-3, 4, -1, 2], 2), 26);
        assert_eq!(Solution::max_score(vec![-3, -2], vec![1, 2], 2), -7);
    }
}
