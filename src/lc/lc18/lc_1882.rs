// https://leetcode.com/problems/process-tasks-using-servers/
// 1882. Process Tasks Using Servers
pub struct Solution;
use std::collections::BinaryHeap;
impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut free_servers: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut busy_servers: BinaryHeap<(i64, i32, i32)> = BinaryHeap::new();

        for (index, &weight) in servers.iter().enumerate() {
            free_servers.push((-weight, -(index as i32)));
        }

        let mut answer = Vec::with_capacity(tasks.len());
        let mut current_time = 0_i64;

        for (task_index, &task_time) in tasks.iter().enumerate() {
            current_time = current_time.max(task_index as i64);

            while let Some(&(free_time, weight, index)) = busy_servers.peek() {
                if -free_time > current_time {
                    break;
                }
                busy_servers.pop();
                free_servers.push((weight, index));
            }

            if free_servers.is_empty() {
                if let Some(&(free_time, _, _)) = busy_servers.peek() {
                    current_time = -free_time;
                }

                while let Some(&(free_time, weight, index)) = busy_servers.peek() {
                    if -free_time != current_time {
                        break;
                    }
                    busy_servers.pop();
                    free_servers.push((weight, index));
                }
            }

            let (weight, index) = free_servers.pop().unwrap();
            answer.push(-index);
            busy_servers.push((-(current_time + task_time as i64), weight, index));
        }

        answer
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_assign_tasks() {
        assert_eq!(
            Solution::assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]),
            vec![2, 2, 0, 2, 1, 2]
        );
        assert_eq!(
            Solution::assign_tasks(vec![5, 1, 4, 3, 2], vec![2, 1, 2, 4, 5, 2, 1]),
            vec![1, 4, 1, 4, 1, 3, 2]
        );
    }
}
