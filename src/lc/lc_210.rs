// https://leetcode.com/problems/course-schedule-ii/
// 210. Course Schedule II
pub struct Solution;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph_pre: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
        let mut graph_rev: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
        for pre in prerequisites {
            graph_pre[pre[0] as usize].push(pre[1] as usize);
            graph_rev[pre[1] as usize].push(pre[0] as usize);
        }
        let mut order = vec![];
        let mut q = std::vec![];
        let mut visited = vec![false; num_courses as usize];
        for i in 0..num_courses as usize {
            if graph_pre[i].is_empty() {
                q.push(i);
            }
        }
        while let Some(i) = q.pop() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            order.push(i as i32);
            for j in graph_rev[i].iter() {
                graph_pre[*j].retain(|&x| x != i);
                if graph_pre[*j].is_empty() {
                    q.push(*j);
                }
            }
        }
        if order.len() == num_courses as usize {
            order
        } else {
            vec![]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_order() {
        assert_eq!(Solution::find_order(2, vec_vec![[1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::find_order(4, vec_vec![[1, 0], [2, 0], [3, 1], [3, 2]]),
            vec![0, 2, 1, 3]
        );
        assert_eq!(Solution::find_order(1, vec![]), vec![0]);
    }
}
