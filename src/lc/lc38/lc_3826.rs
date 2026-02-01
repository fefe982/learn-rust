// https://leetcode.com/problems/minimum-partition-score/
// 3826. Minimum Partition Score
pub struct Solution;
impl Solution {
    pub fn min_partition_score(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut sum = vec![0; n + 1];
        let mut f = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + nums[i] as i64;
            f[i + 1] = sum[i + 1] * (sum[i + 1] + 1) / 2;
        }
        for kk in 2..=k as usize {
            let mut q = std::collections::VecDeque::new();
            q.push_back((-sum[kk - 1], (sum[kk - 1] * sum[kk - 1] - sum[kk - 1]) / 2 + f[kk - 1]));
            let mut nf = vec![0; n + 1];
            for i in kk..=n {
                while q.len() > 1 {
                    let y0 = q[0].0 * sum[i] + q[0].1;
                    let y1 = q[1].0 * sum[i] + q[1].1;
                    if y0 >= y1 {
                        q.pop_front();
                    } else {
                        break;
                    }
                }
                nf[i] = q[0].0 * sum[i] + q[0].1 + (sum[i] + 1) * sum[i] / 2;
                let l3 = (-sum[i], (sum[i] * sum[i] - sum[i]) / 2 + f[i]);
                while q.len() > 1 {
                    let l2 = q[q.len() - 1];
                    let l1 = q[q.len() - 2];
                    if (l3.1 - l2.1) * (l1.0 - l2.0) <= (l2.1 - l1.1) * (l2.0 - l3.0) {
                        q.pop_back();
                    } else {
                        break;
                    }
                }
                q.push_back(l3);
            }
            f = nf;
        }
        f[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_partition_score() {
        assert_eq!(Solution::min_partition_score(vec![5, 1, 2, 1], 2), 25);
        assert_eq!(Solution::min_partition_score(vec![1, 2, 3, 4], 1), 55);
    }
}
