// https://leetcode.com/problems/minimum-increments-to-equalize-leaf-paths/
// 3593. Minimum Increments to Equalize Leaf Paths
pub struct Solution;
impl Solution {
    fn dfs(tree: &Vec<Vec<usize>>, node: usize, parent: usize, cost: &Vec<i32>) -> (i32, i32) {
        let mut v = vec![];
        let mut inc = 0;
        for &c in &tree[node] {
            if c == parent {
                continue;
            }
            let (cv, ci) = Self::dfs(tree, c, node, cost);
            v.push(cv);
            inc += ci;
        }
        if v.len() == 0 {
            return (cost[node], 0);
        }
        v.sort();
        let max = v[v.len() - 1];
        for i in 0..v.len() - 1 {
            if v[i] < max {
                inc += 1;
            } else {
                break;
            }
        }
        (max + cost[node], inc)
    }
    pub fn min_increase(n: i32, edges: Vec<Vec<i32>>, cost: Vec<i32>) -> i32 {
        let mut tree = vec![vec![]; n as usize];
        for e in edges {
            tree[e[0] as usize].push(e[1] as usize);
            tree[e[1] as usize].push(e[0] as usize);
        }
        Self::dfs(&tree, 0, usize::MAX, &cost).1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_increase() {
        assert_eq!(Solution::min_increase(3, vec_vec![[1, 0], [0, 2]], vec![2, 1, 3]), 1);
        assert_eq!(Solution::min_increase(3, vec_vec![[0, 1], [0, 2]], vec![2, 1, 3]), 1);
        assert_eq!(Solution::min_increase(3, vec_vec![[0, 1], [1, 2]], vec![5, 1, 4]), 0);
        assert_eq!(
            Solution::min_increase(5, vec_vec![[0, 4], [0, 1], [1, 2], [1, 3]], vec![3, 4, 1, 1, 7]),
            1
        );
    }
}
