// https://leetcode.com/problems/the-employee-that-worked-on-the-longest-task/
// 2432. The Employee That Worked on the Longest Task
pub struct Solution;
impl Solution {
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut last = 0;
        let mut max_time = 0;
        let mut id = 0;
        for log in logs {
            let time = log[1] - last;
            last = log[1];
            if time > max_time {
                max_time = time;
                id = log[0];
            } else if time == max_time && log[0] < id {
                id = log[0];
            }
        }
        id
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn hardest_worker() {
        assert_eq!(
            Solution::hardest_worker(10, vec_vec![[0, 3], [2, 5], [0, 9], [1, 15]]),
            1
        );
        assert_eq!(
            Solution::hardest_worker(26, vec_vec![[1, 1], [3, 7], [2, 12], [7, 17]]),
            3
        );
        assert_eq!(Solution::hardest_worker(2, vec_vec![[0, 10], [1, 20]]), 0);
    }
}
