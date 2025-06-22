// https://leetcode.com/problems/course-schedule/
// 207. Course Schedule
pub struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut edge_out = vec![std::collections::HashSet::<i32>::new(); num_courses as usize];
        let mut edge_in = edge_out.clone();
        for p in prerequisites {
            edge_in[p[1] as usize].insert(p[0]);
            edge_out[p[0] as usize].insert(p[1]);
        }
        let mut q = std::collections::VecDeque::new();
        let mut cnt = 0;
        for (e, i) in edge_in.iter().zip(0..) {
            if e.is_empty() {
                q.push_back(i);
            }
        }
        while let Some(from) = q.pop_front() {
            cnt += 1;
            for &to in &edge_out[from as usize] {
                edge_in[to as usize].remove(&from);
                if edge_in[to as usize].is_empty() {
                    q.push_back(to);
                }
            }
        }
        cnt == num_courses
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn can_finish() {
        assert_eq!(Solution::can_finish(2, vec_vec![[1, 0]]), true);
        assert_eq!(Solution::can_finish(2, vec_vec![[1, 0], [0, 1]]), false);
    }
}
