// https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/
// 2322. Minimum Score After Removals on a Tree
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(
        g: &Vec<Vec<usize>>,
        nums: &Vec<i32>,
        children: &mut Vec<Rc<RefCell<std::collections::HashSet<usize>>>>,
        val: &mut Vec<i32>,
        u: usize,
        p: usize,
    ) {
        for &v in &g[u] {
            if v == p {
                continue;
            }
            Self::dfs(g, nums, children, val, v, u);
            children[u].borrow_mut().insert(v);
            children[u].borrow_mut().extend(children[v].borrow().iter());
            val[u] ^= val[v];
        }
        children[u].borrow_mut().insert(u);
        val[u] ^= nums[u];
    }
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![]; nums.len()];
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut children = vec![];
        for _ in 0..nums.len() {
            children.push(Rc::new(RefCell::new(std::collections::HashSet::<usize>::new())));
        }
        let mut val = vec![0; nums.len()];
        Self::dfs(&g, &nums, &mut children, &mut val, 0, usize::MAX);
        let mut edges = edges;
        let mut min = i32::MAX;
        let value = |v1: i32, v2: i32, v3: i32| v1.max(v2).max(v3) - v1.min(v2).min(v3);
        for i in 0..edges.len() {
            let mut si = edges[i][0] as usize;
            let mut ei = edges[i][1] as usize;
            if !children[si].borrow().contains(&ei) {
                std::mem::swap(&mut si, &mut ei);
            }
            edges[i][0] = si as i32;
            edges[i][1] = ei as i32;
            for j in 0..i {
                let sj = edges[j][0] as usize;
                let ej = edges[j][1] as usize;
                if children[ei].borrow().contains(&sj) {
                    min = min.min(value(val[0] ^ val[ei], val[ei] ^ val[ej], val[ej]));
                } else if children[ej].borrow().contains(&si) {
                    min = min.min(value(val[0] ^ val[ej], val[ej] ^ val[ei], val[ei]));
                } else {
                    min = min.min(value(val[0] ^ val[ei] ^ val[ej], val[ej], val[ei]));
                }
            }
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_score() {
        assert_eq!(
            Solution::minimum_score(vec![1, 5, 5, 4, 11], vec_vec![[0, 1], [1, 2], [1, 3], [3, 4]]),
            9
        );
        assert_eq!(
            Solution::minimum_score(vec![5, 5, 2, 4, 4, 2], vec_vec![[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]]),
            0
        );
    }
}
