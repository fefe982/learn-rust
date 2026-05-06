// https://leetcode.com/problems/count-nodes-with-the-highest-score/
// 2049. Count Nodes With the Highest Score
pub struct Solution;
impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let n = parents.len();
        let mut children = vec![Vec::new(); n];
        for (i, &p) in parents.iter().enumerate().skip(1) {
            children[p as usize].push(i);
        }

        // Build a traversal order, then process in reverse for postorder.
        let mut order = Vec::with_capacity(n);
        let mut stack = vec![0_usize];
        while let Some(u) = stack.pop() {
            order.push(u);
            for &v in &children[u] {
                stack.push(v);
            }
        }

        let mut size = vec![1_usize; n];
        for &u in order.iter().rev() {
            for &v in &children[u] {
                size[u] += size[v];
            }
        }

        let mut best = 0_u128;
        let mut ans = 0_i32;
        for u in 0..n {
            let mut score = 1_u128;
            for &v in &children[u] {
                score *= size[v] as u128;
            }
            let rest = n - size[u];
            if rest > 0 {
                score *= rest as u128;
            }

            if score > best {
                best = score;
                ans = 1;
            } else if score == best {
                ans += 1;
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_highest_score_nodes() {
        assert_eq!(Solution::count_highest_score_nodes(vec![-1, 2, 0, 2, 0]), 3);
        assert_eq!(Solution::count_highest_score_nodes(vec![-1, 2, 0]), 2);
    }
}
