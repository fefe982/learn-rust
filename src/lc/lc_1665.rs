// https://leetcode.com/problems/minimum-initial-energy-to-finish-tasks/
// 1665. Minimum Initial Energy to Finish Tasks
pub struct Solution;
impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_by_key(|x| std::cmp::Reverse(x[1] - x[0]));
        let mut ans = 0;
        let mut e = 0;
        for t in tasks {
            if e < t[1] {
                let d = t[1] - e;
                ans += d;
                e += d;
            }
            e -= t[0];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_effort() {
        assert_eq!(Solution::minimum_effort(vec_vec![[1, 2], [2, 4], [4, 8]]), 8);
        assert_eq!(
            Solution::minimum_effort(vec_vec![[1, 3], [2, 4], [10, 11], [10, 12], [8, 9]]),
            32
        );
        assert_eq!(
            Solution::minimum_effort(vec_vec![[1, 7], [2, 8], [3, 9], [4, 10], [5, 11], [6, 12]]),
            27
        );
    }
}
