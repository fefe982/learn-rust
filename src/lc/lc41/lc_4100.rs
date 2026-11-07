// https://leetcode.com/problems/minimum-cost-path-with-even-xor-parity/
// 4100. Minimum Cost Path With Even XOR Parity
pub struct Solution;
impl Solution {
    pub fn min_activation_cost(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i32 {
        let n = edges.len() + 1;
        let mut min_val = vec![i32::MAX; n];
        let mut ans = i32::MAX;
        let values = values.into_iter().map(|x| x.count_ones() % 2).collect::<Vec<_>>();
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            if values[u] == values[v] {
                ans = ans.min(w);
            } else {
                let z = if values[u] == 0 { u } else { v };
                if min_val[z] != i32::MAX {
                    ans = ans.min(w + min_val[z]);
                }
                min_val[z] = min_val[z].min(w);
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_activation_cost() {
        assert_eq!(
            Solution::min_activation_cost(
                vec_vec![[0, 1, 4], [0, 2, 6], [0, 3, 3], [3, 4, 5]],
                vec![3, 1, 7, 1, 3]
            ),
            7
        );
        assert_eq!(
            Solution::min_activation_cost(vec_vec![[0, 1, 5], [1, 2, 10]], vec![1, 7, 3]),
            5
        );
        assert_eq!(
            Solution::min_activation_cost(vec_vec![[0, 1, 5], [0, 2, 7], [0, 3, 9]], vec![1, 3, 5, 9]),
            -1
        );
    }
}
