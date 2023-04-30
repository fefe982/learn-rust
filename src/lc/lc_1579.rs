// https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/
// 1579. Remove Max Number of Edges to Keep Graph Fully Traversable
pub struct Solution;
impl Solution {
    fn union(p: &mut Vec<usize>, s: &mut Vec<i32>, id1: usize, id2: usize) -> bool {
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
            true
        } else {
            false
        }
    }
    fn get_id(p: &mut Vec<usize>, id: usize) -> usize {
        if p[id] == id {
            return id;
        }
        let pid = p[id];
        if p[pid] == pid {
            return pid;
        }
        let pid = Self::get_id(p, p[id]);
        p[id] = pid;
        pid
    }
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut p1: Vec<_> = (0..n).collect();
        let mut s1 = vec![1; n];
        let mut remove = 0;
        let mut e1 = 0;
        for e in &edges {
            if e[0] == 3 {
                if !Self::union(&mut p1, &mut s1, e[1] as usize - 1, e[2] as usize - 1) {
                    remove += 1;
                } else {
                    e1 += 1;
                }
            }
        }
        let mut p2 = p1.clone();
        let mut s2 = s1.clone();
        let mut e2 = e1;
        for e in &edges {
            if e[0] == 1 {
                if !Self::union(&mut p1, &mut s1, e[1] as usize - 1, e[2] as usize - 1) {
                    remove += 1;
                } else {
                    e1 += 1;
                }
            } else if e[0] == 2 {
                if !Self::union(&mut p2, &mut s2, e[1] as usize - 1, e[2] as usize - 1) {
                    remove += 1;
                } else {
                    e2 += 1;
                }
            }
        }
        if e1 == n - 1 && e2 == n - 1 {
            remove
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;
    #[test]
    fn max_num_edges_to_remove() {
        assert_eq!(
            Solution::max_num_edges_to_remove(
                4,
                vec_vec![
                    [3, 1, 2],
                    [3, 2, 3],
                    [1, 1, 3],
                    [1, 2, 4],
                    [1, 1, 2],
                    [2, 3, 4]
                ]
            ),
            2
        );
        assert_eq!(
            Solution::max_num_edges_to_remove(
                4,
                vec_vec![[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]]
            ),
            0
        );
        assert_eq!(
            Solution::max_num_edges_to_remove(4, vec_vec![[3, 2, 3], [1, 1, 2], [2, 3, 4]]),
            -1
        );
    }
}
