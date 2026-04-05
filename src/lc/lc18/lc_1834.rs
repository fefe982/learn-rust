// https://leetcode.com/problems/single-threaded-cpu/
// 1834. Single-Threaded CPU
pub struct Solution;
impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut tasks = tasks
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v[0], v[1], i as i32))
            .collect::<Vec<_>>();
        tasks.sort_by_key(|k| (k.0, k.1));
        let mut ans = vec![];
        let mut heap = BinaryHeap::new();
        let mut time = 0;
        let mut i = 0;
        while i < tasks.len() {
            while i < tasks.len() && tasks[i].0 <= time {
                heap.push(Reverse((tasks[i].1, tasks[i].2)));
                i += 1;
            }
            if let Some(Reverse((t, j))) = heap.pop() {
                time += t;
                ans.push(j);
            } else if i < tasks.len() {
                time = tasks[i].0;
            }
        }
        while !heap.is_empty() {
            let Reverse((_, i)) = heap.pop().unwrap();
            ans.push(i);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn get_order() {
        assert_eq!(
            Solution::get_order(vec_vec![[1, 2], [2, 4], [3, 2], [4, 1]]),
            [0, 2, 3, 1]
        );
        assert_eq!(
            Solution::get_order(vec_vec![[7, 10], [7, 12], [7, 5], [7, 4], [7, 2]]),
            [4, 3, 2, 0, 1]
        );
    }
}
