// https://leetcode.com/problems/course-schedule-iv/
// 1462. Course Schedule IV
pub struct Solution;
impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut graph = [0_i128; 100];
        let mut deg = [0; 100];
        for prerequisite in prerequisites {
            graph[prerequisite[1] as usize] |= 1 << prerequisite[0];
            deg[prerequisite[0] as usize] += 1;
        }
        let mut q = std::collections::VecDeque::new();
        for i in 0..num_courses {
            if deg[i as usize] == 0 {
                q.push_back(i);
            }
        }
        let mut nxt = [0_i128; 100];
        while let Some(i) = q.pop_front() {
            let prei = graph[i as usize];
            if prei == 0 {
                continue;
            }
            for j in 0..num_courses {
                if prei & (1 << j) != 0 {
                    nxt[j as usize] |= 1 << i;
                    nxt[j as usize] |= nxt[i as usize];
                    deg[j as usize] -= 1;
                    if deg[j as usize] == 0 {
                        q.push_back(j);
                    }
                }
            }
        }
        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            result.push(if nxt[query[0] as usize] & (1 << query[1]) != 0 {
                true
            } else {
                false
            });
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_if_prerequisite() {
        assert_eq!(
            Solution::check_if_prerequisite(
                5,
                vec_vec![[0, 1], [1, 2], [2, 3], [3, 4]],
                vec_vec![[0, 4], [4, 0], [1, 3], [3, 0]]
            ),
            vec![true, false, true, false]
        );
        assert_eq!(
            Solution::check_if_prerequisite(2, vec_vec![[1, 0]], vec_vec![[0, 1], [1, 0]]),
            vec![false, true]
        );
        assert_eq!(
            Solution::check_if_prerequisite(2, vec_vec![], vec_vec![[0, 1], [1, 0]]),
            vec![false, false]
        );
        assert_eq!(
            Solution::check_if_prerequisite(3, vec_vec![[1, 2], [1, 0], [2, 0]], vec_vec![[1, 0], [1, 2]]),
            vec![true, true]
        );
    }
}
