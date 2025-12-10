// https://leetcode.cn/problems/t3fKg1/
// LCP 32. 批量处理任务
pub struct Solution;
impl Solution {
    pub fn process_tasks(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_unstable_by_key(|x| x[1]);
        let mut q = vec![(-2, -2, 0)];
        for task in tasks {
            let idx = q.partition_point(|x| x.0 < task[0]);
            let mut d = task[2];
            d -= q.last().unwrap().2 - q[idx - 1].2;
            if q[idx - 1].1 >= task[0] {
                d -= q[idx - 1].1 - task[0] + 1;
            }
            if d <= 0 {
                continue;
            }
            while task[1] - q.last().unwrap().1 <= d {
                let last = q.pop().unwrap();
                d += last.1 - last.0 + 1;
            }
            let s = q.last().unwrap().2;
            q.push((task[1] - d + 1, task[1], s + d));
        }
        q.last().unwrap().2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn process_tasks() {
        assert_eq!(Solution::process_tasks(vec_vec![[0, 0, 1], [1, 1, 1], [2, 2, 1]]), 3);
        assert_eq!(Solution::process_tasks(vec_vec![[1, 3, 2], [2, 5, 3], [5, 6, 2]]), 4);
        assert_eq!(Solution::process_tasks(vec_vec![[2, 3, 1], [5, 5, 1], [5, 6, 2]]), 3);
    }
}
