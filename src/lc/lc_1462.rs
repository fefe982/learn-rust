// https://leetcode.com/problems/course-schedule-iv/
// 1462. Course Schedule IV
pub struct Solution;
impl Solution {
    fn check_if_prerequisite_helper(
        pre: i32,
        cur: i32,
        visited: &mut Vec<bool>,
        graph: &mut std::collections::HashMap<i32, std::collections::HashSet<i32>>,
    ) -> bool {
        if visited[pre as usize] {
            return graph
                .get(&pre)
                .and_then(|s| Some(s.contains(&cur)))
                .unwrap_or(false);
        }
        visited[pre as usize] = true;
        let pre_set = graph.remove(&pre);
        if let Some(mut pre_set) = pre_set {
            let mut v = vec![];
            for &next in pre_set.iter() {
                Self::check_if_prerequisite_helper(next, cur, visited, graph);
            }
            for &next in pre_set.iter() {
                if let Some(ns) = graph.get(&next) {
                    for &nn in ns {
                        v.push(nn);
                    }
                }
            }
            for nn in v {
                pre_set.insert(nn);
            }
            graph.insert(pre, pre_set);
            return graph.get(&pre).unwrap().contains(&cur);
        } else {
            false
        }
    }
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut graph = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
        for prerequisite in prerequisites {
            graph
                .entry(prerequisite[0])
                .or_default()
                .insert(prerequisite[1]);
        }
        let mut visited = vec![false; num_courses as usize];
        let mut result = vec![];
        for query in queries {
            result.push(Solution::check_if_prerequisite_helper(
                query[0],
                query[1],
                &mut visited,
                &mut graph,
            ));
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
            Solution::check_if_prerequisite(
                3,
                vec_vec![[1, 2], [1, 0], [2, 0]],
                vec_vec![[1, 0], [1, 2]]
            ),
            vec![true, true]
        );
    }
}
