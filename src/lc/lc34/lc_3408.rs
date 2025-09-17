// https://leetcode.com/problems/design-task-manager/
// 3408. Design Task Manager
use std::collections::BinaryHeap;
use std::collections::HashMap;
pub struct TaskManager {
    pq: BinaryHeap<(i32, i32)>,
    map: HashMap<i32, (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    pub fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut pq = BinaryHeap::new();
        let mut map = HashMap::new();
        for task in tasks {
            pq.push((task[2], task[1]));
            map.insert(task[1], (task[0], task[2]));
        }
        Self { pq, map }
    }

    pub fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.pq.push((priority, task_id));
        self.map.insert(task_id, (user_id, priority));
    }

    pub fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(&(user_id, priority)) = self.map.get(&task_id) {
            if new_priority != priority {
                self.map.insert(task_id, (user_id, new_priority));
                self.pq.push((new_priority, task_id));
            }
        }
    }

    pub fn rmv(&mut self, task_id: i32) {
        self.map.remove(&task_id);
    }

    pub fn exec_top(&mut self) -> i32 {
        while let Some((priority, task_id)) = self.pq.pop() {
            if let Some(&(user_id, p)) = self.map.get(&task_id) {
                if p == priority {
                    self.map.remove(&task_id);
                    return user_id;
                }
            }
        }
        -1
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn it_works() {
        let mut obj = TaskManager::new(vec_vec![[1, 101, 10], [2, 102, 20], [3, 103, 15]]);
        obj.add(4, 104, 5);
        obj.edit(102, 8);
        assert_eq!(obj.exec_top(), 3);
        obj.rmv(101);
        obj.add(5, 105, 15);
        assert_eq!(obj.exec_top(), 5);
    }
}
