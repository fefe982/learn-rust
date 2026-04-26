// https://leetcode.com/problems/count-connected-subgraphs-with-even-node-sum/
// 3910. Count Connected Subgraphs With Even Node Sum
pub struct Solution;
impl Solution {
    pub fn even_sum_subgraphs(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut adj = vec![0u16; n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            adj[u] |= 1u16 << v;
            adj[v] |= 1u16 << u;
        }

        let mut ans = 0i32;
        let limit = 1u16 << n;
        for mask in 1u16..limit {
            let mut parity = 0i32;
            for (idx, &value) in nums.iter().enumerate() {
                if ((mask >> idx) & 1) == 1 {
                    parity ^= value;
                }
            }
            if parity != 0 {
                continue;
            }

            let start = mask.trailing_zeros() as usize;
            let mut seen = 0u16;
            let mut stack = vec![start];
            while let Some(u) = stack.pop() {
                let bit = 1u16 << u;
                if (seen & bit) != 0 {
                    continue;
                }
                seen |= bit;

                let mut neighbors = adj[u] & mask & !seen;
                while neighbors != 0 {
                    let v = neighbors.trailing_zeros() as usize;
                    stack.push(v);
                    neighbors &= neighbors - 1;
                }
            }

            if seen == mask {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn even_sum_subgraphs() {
        assert_eq!(Solution::even_sum_subgraphs(vec![1, 0, 1], vec_vec![[0, 1], [1, 2]]), 2);
        assert_eq!(Solution::even_sum_subgraphs(vec![1], vec![]), 0);
    }
}
