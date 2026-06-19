// https://leetcode.com/problems/task-scheduler-ii/
// 2365. Task Scheduler II
pub struct Solution;
impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let mut last = std::collections::HashMap::new();
        let mut time = 0;
        let space = space as i64;
        for task in tasks {
            if let Some(&t) = last.get(&task) {
                time = time.max(t + space);
            }
            time += 1;
            last.insert(task, time);
        }
        time
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task_scheduler_ii() {
        assert_eq!(Solution::task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3), 9);
        assert_eq!(Solution::task_scheduler_ii(vec![5, 8, 8, 5], 2), 6);
    }
}
