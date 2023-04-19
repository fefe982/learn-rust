// https://leetcode.com/problems/partition-array-for-maximum-sum/
// 1043. Partition Array for Maximum Sum
pub struct Solution;
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut max_vec = vec![vec![]; arr.len()];
        let mut cum_sum = vec![0; arr.len() + 1];
        for (idx, &i) in arr.iter().enumerate() {
            max_vec[idx].push(i);
            cum_sum[idx + 1] = cum_sum[idx] + i;
            for i in 1..k {
                if idx + i < arr.len() {
                    let m = std::cmp::max(max_vec[idx][i - 1], arr[idx + i]);
                    max_vec[idx].push(m);
                } else {
                    break;
                }
            }
        }
        let mut max_res = vec![0; arr.len()];
        for idx in 0..k {
            max_res[idx] = max_vec[0][idx] * (idx as i32 + 1);
        }
        for idx in k..arr.len() {
            for i in 0..k {
                max_res[idx] = std::cmp::max(
                    max_res[idx],
                    max_res[idx - i - 1] + max_vec[idx - i][i] * (i as i32 + 1),
                )
            }
        }
        max_res[arr.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum_after_partitioning() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
            84
        );
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
            83
        );
        assert_eq!(Solution::max_sum_after_partitioning(vec![1], 1), 1);
    }
}
