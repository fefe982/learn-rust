// https://leetcode.com/problems/redundant-connection-ii/
// 685. Redundant Connection II
pub struct Solution;
impl Solution {
    fn find_p(p: &Vec<usize>, mut i: usize, target: usize) -> usize {
        while p[i] != 0 && p[i] != target {
            i = p[i];
        }
        p[i]
    }
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut p = vec![0; edges.len() + 1];
        let mut pos = vec![0; edges.len() + 1];
        let mut t_node = 0;
        let mut t_p = 0;
        let mut t_pos = 0;
        for (i, edge) in edges.iter().enumerate() {
            let pe = edge[0] as usize;
            let ce = edge[1] as usize;
            if p[ce] == 0 {
                p[ce] = pe;
                pos[ce] = i;
            } else {
                t_p = pe;
                t_node = ce;
                t_pos = i;
            }
        }
        if t_pos != 0 {
            let b1 = Self::find_p(&p, t_p, t_node);
            let b2 = Self::find_p(&p, p[t_node], t_node);
            if b1 == b2 {
                if t_pos < pos[t_node] {
                    return vec![p[t_node] as i32, t_node as i32];
                } else {
                    return vec![t_p as i32, t_node as i32];
                }
            }
            if b1 == 0 {
                return vec![p[t_node] as i32, t_node as i32];
            }
            return vec![t_p as i32, t_node as i32];
        }
        let mut v = vec![false; edges.len() + 1];
        let mut i = 1;
        while !v[i] {
            v[i] = true;
            i = p[i];
        }
        let mut j = p[i];
        let mut max_pos = pos[i];
        while j != i {
            max_pos = max_pos.max(pos[j]);
            j = p[j];
        }
        edges[max_pos].clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_redundant_directed_connection() {
        assert_eq!(
            Solution::find_redundant_directed_connection(vec_vec![[1, 2], [1, 3], [2, 3]]),
            vec![2, 3]
        );
        assert_eq!(
            Solution::find_redundant_directed_connection(vec_vec![[1, 2], [2, 3], [3, 4], [4, 1], [1, 5]]),
            vec![4, 1]
        );
    }
}
