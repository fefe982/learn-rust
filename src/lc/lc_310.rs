// https://leetcode.com/problems/minimum-height-trees/
// 310. Minimum Height Trees
pub struct Solution;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        let mut cnt = vec![0; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            cnt[e[0] as usize] += 1;
            g[e[1] as usize].push(e[0] as usize);
            cnt[e[1] as usize] += 1;
        }
        let mut q = vec![];
        let mut visited = vec![false; n];
        let mut total = n;
        for i in 0..n {
            if cnt[i] <= 1 {
                visited[i] = true;
                q.push(i);
                total -= 1;
            }
        }
        while total > 0 {
            let mut next_q = vec![];
            for i in q.iter() {
                for &j in g[*i].iter() {
                    if visited[j] {
                        continue;
                    }
                    cnt[j] -= 1;
                    if cnt[j] == 1 {
                        visited[j] = true;
                        total -= 1;
                        next_q.push(j);
                    }
                }
            }
            q = next_q;
        }
        q.into_iter().map(|i| i as i32).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_max_length() {
        assert_eq!(Solution::find_min_height_trees(1, vec_vec![]), [0]);
        assert_eq!(
            Solution::find_min_height_trees(4, vec_vec![[1, 0], [1, 2], [1, 3]]),
            [1]
        );
        assert_eq!(
            Solution::find_min_height_trees(6, vec_vec![[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]),
            [3, 4]
        );
    }
}
