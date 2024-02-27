// https://leetcode.cn/problems/count-valid-paths-in-a-tree/
// 2867. Count Valid Paths in a Tree
pub struct Solution;
impl Solution {
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut isprime = vec![true; n as usize + 1];
        isprime[0] = false;
        isprime[1] = false;
        let mut primes = vec![];
        for i in 2..=n as usize {
            if isprime[i] {
                primes.push(i);
                let mut j = 2;
                while i * j <= n as usize {
                    isprime[i * j] = false;
                    j += 1;
                }
            }
        }
        let mut graph = vec![vec![]; n as usize + 1];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        let mut tree_sz = vec![0; n as usize + 1];
        let mut ans = 0;
        for pr in primes {
            let mut sub_tree_size = 0;
            for &n in &graph[pr] {
                if isprime[n as usize] {
                    continue;
                }
                let mut sz = 1;
                if tree_sz[n as usize] > 0 {
                    sz = tree_sz[n as usize];
                } else {
                    let mut q = vec![(pr, n)];
                    while let Some((p, c)) = q.pop() {
                        for &n in &graph[c] {
                            if n == p {
                                continue;
                            }
                            if isprime[n] {
                                continue;
                            }
                            sz += 1;
                            q.push((c, n));
                        }
                    }
                    tree_sz[n as usize] = sz;
                }
                ans += (sub_tree_size as i64 + 1) * sz as i64;
                sub_tree_size += sz;
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
    fn test_count_paths() {
        assert_eq!(Solution::count_paths(5, vec_vec![[1, 2], [1, 3], [2, 4], [2, 5]]), 4);
        assert_eq!(
            Solution::count_paths(6, vec_vec![[1, 2], [1, 3], [2, 4], [3, 5], [3, 6]]),
            6
        );
    }
}
