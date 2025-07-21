// https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/
// 1697. Checking Existence of Edge Length Limited Paths
pub struct Solution;
impl Solution {
    fn union(p: &mut Vec<usize>, s: &mut Vec<i32>, id1: usize, id2: usize) {
        let pid1 = Self::get_id(p, id1);
        let pid2 = Self::get_id(p, id2);
        if pid1 != pid2 {
            if s[pid1] > s[pid2] {
                s[pid1] += s[pid2];
                p[pid2] = pid1;
            } else {
                s[pid2] += s[pid1];
                p[pid1] = pid2;
            }
        }
    }
    fn get_id(p: &mut Vec<usize>, id: usize) -> usize {
        if p[id] == id {
            return id;
        }
        let pid = Self::get_id(p, p[id]);
        p[id] = pid;
        pid
    }
    pub fn distance_limited_paths_exist(
        n: i32,
        mut edge_list: Vec<Vec<i32>>,
        mut queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut p: Vec<_> = (0..n).collect();
        let mut s = vec![1; n];
        let mut result = vec![false; queries.len()];
        edge_list.sort_by_key(|x| x[2]);
        queries
            .iter_mut()
            .enumerate()
            .for_each(|x| x.1.push(x.0 as i32));
        queries.sort_by_key(|x| x[2]);
        let mut idx = 0;
        for q in queries {
            while idx < edge_list.len() && edge_list[idx][2] < q[2] {
                Self::union(
                    &mut p,
                    &mut s,
                    edge_list[idx][0] as usize,
                    edge_list[idx][1] as usize,
                );
                idx += 1;
            }
            let pid1 = Self::get_id(&mut p, q[0] as usize);
            let pid2 = Self::get_id(&mut p, q[1] as usize);
            result[q[3] as usize] = pid1 == pid2;
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;
    #[test]
    fn distance_limited_paths_exist() {
        assert_eq!(
            Solution::distance_limited_paths_exist(
                3,
                vec_vec![[0, 1, 2], [1, 2, 4], [2, 0, 8], [1, 0, 16]],
                vec_vec![[0, 1, 2], [0, 2, 5]]
            ),
            vec![false, true]
        );
        assert_eq!(
            Solution::distance_limited_paths_exist(
                5,
                vec_vec![[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]],
                vec_vec![[0, 4, 14], [1, 4, 13]]
            ),
            vec![true, false]
        );
    }
}
