// https://leetcode.com/problems/minimize-maximum-component-cost/
// 3613. Minimize Maximum Component Cost
pub struct Solution;
impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if k == n {
            return 0;
        }
        let mut edges = edges;
        edges.sort_unstable_by_key(|e| e[2]);
        let n = n as usize;
        let mut fa = (0..n).collect();
        fn find(x: usize, fa: &mut Vec<usize>) -> usize {
            if fa[x] != x {
                fa[x] = find(fa[x], fa);
            }
            fa[x]
        }
        fn union(x: usize, y: usize, fa: &mut Vec<usize>) -> bool {
            let x = find(x, fa);
            let y = find(y, fa);
            if x != y {
                fa[x] = y;
                true
            } else {
                false
            }
        }
        let mut cnt = n;
        let k = k as usize;
        for edge in edges {
            if union(edge[0] as usize, edge[1] as usize, &mut fa) {
                cnt -= 1;
                if cnt <= k {
                    return edge[2];
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_cost() {
        assert_eq!(
            Solution::min_cost(5, vec_vec![[0, 1, 4], [1, 2, 3], [1, 3, 2], [3, 4, 6]], 2),
            4
        );
        assert_eq!(Solution::min_cost(4, vec_vec![[0, 1, 5], [1, 2, 5], [2, 3, 5]], 1), 5);
    }
}
