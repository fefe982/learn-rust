// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/
// 1335. Minimum Difficulty of a Job Schedule
pub struct Solution;
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        if job_difficulty.len() < d {
            return -1;
        }
        let mut arr = vec![vec![0; job_difficulty.len() - d + 1]; d];
        let mut md = 0;
        for ij in (d - 1..job_difficulty.len()).rev() {
            md = std::cmp::max(md, job_difficulty[ij]);
            arr[d - 1][ij + 1 - d] = md;
        }
        for id in (0..d - 1).rev() {
            for ij in id..job_difficulty.len() - d + id + 1 {
                let mut m = i32::MAX;
                md = 0;
                for ij2 in ij..job_difficulty.len() - d + id + 1 {
                    md = std::cmp::max(md, job_difficulty[ij2]);
                    m = std::cmp::min(m, md + arr[id + 1][ij2 - id]);
                }
                arr[id][ij - id] = m;
            }
        }
        arr[0][0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_difficulty() {
        assert_eq!(
            Solution::min_difficulty(vec![11, 111, 22, 222, 33, 333, 44, 444], 6),
            843
        );
        assert_eq!(Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2), 7);
        assert_eq!(Solution::min_difficulty(vec![9, 9, 9], 4), -1);
        assert_eq!(Solution::min_difficulty(vec![1, 1, 1], 3), 3);
    }
}
