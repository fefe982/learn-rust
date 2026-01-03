// https://leetcode.com/problems/longest-palindromic-path-in-graph/
// 3615. Longest Palindromic Path in a Binary Tree
pub struct Solution;
impl Solution {
    pub fn max_len(n: i32, edges: Vec<Vec<i32>>, label: String) -> i32 {
        let n = n as usize;
        let ne = edges.len();
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push(edge[1] as usize);
            g[edge[1] as usize].push(edge[0] as usize);
        }
        let label: Vec<char> = label.chars().collect();
        fn dfs(
            g: &Vec<Vec<usize>>,
            label: &Vec<char>,
            mut u: usize,
            mut v: usize,
            mask: usize,
            cache: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if u > v {
                std::mem::swap(&mut u, &mut v);
            }
            if cache[u][v][mask] != -1 {
                return cache[u][v][mask];
            }
            let mut ans = 0;
            for &nu in &g[u] {
                if mask & (1 << nu) != 0 {
                    continue;
                }
                for &nv in &g[v] {
                    if mask & (1 << nv) != 0 || nu == nv || label[nu] != label[nv] {
                        continue;
                    }
                    ans = ans.max(dfs(g, label, nu, nv, mask | (1 << nu) | (1 << nv), cache) + 2);
                }
            }
            cache[u][v][mask] = ans;
            ans
        }
        let mut cnt = vec![0; 26];
        for &c in &label {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        let mut nodd = 0;
        for c in cnt {
            if c % 2 == 1 {
                nodd += 1;
            }
        }
        let max = n as i32 - (nodd - 1).max(0);
        if ne == n * (n - 1) / 2 {
            return max;
        }
        let mut cache = vec![vec![vec![-1; 1 << n]; n]; n];
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(dfs(&g, &label, i, i, 1 << i, &mut cache) + 1);
            if ans == max {
                return ans;
            }
            for &j in &g[i] {
                if i < j && label[i] == label[j] {
                    ans = ans.max(dfs(&g, &label, i, j, 1 << i | 1 << j, &mut cache) + 2);
                    if ans == max {
                        return ans;
                    }
                }
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
    fn max_len() {
        assert_eq!(
            Solution::max_len(
                14,
                vec_vec![
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [0, 5],
                    [0, 6],
                    [0, 7],
                    [0, 8],
                    [0, 9],
                    [0, 10],
                    [0, 11],
                    [1, 2],
                    [1, 3],
                    [1, 4],
                    [1, 5],
                    [1, 6],
                    [1, 7],
                    [1, 8],
                    [1, 9],
                    [1, 10],
                    [1, 11],
                    [2, 3],
                    [2, 4],
                    [2, 5],
                    [2, 6],
                    [2, 7],
                    [2, 8],
                    [2, 9],
                    [2, 10],
                    [2, 11],
                    [3, 4],
                    [3, 5],
                    [3, 6],
                    [3, 7],
                    [3, 8],
                    [3, 9],
                    [3, 10],
                    [3, 11],
                    [4, 5],
                    [4, 6],
                    [4, 7],
                    [4, 8],
                    [4, 9],
                    [4, 10],
                    [4, 11],
                    [5, 6],
                    [5, 7],
                    [5, 8],
                    [5, 9],
                    [5, 10],
                    [5, 11],
                    [6, 7],
                    [6, 8],
                    [6, 9],
                    [6, 10],
                    [6, 11],
                    [7, 8],
                    [7, 9],
                    [7, 10],
                    [7, 11],
                    [8, 9],
                    [8, 10],
                    [8, 11],
                    [9, 10],
                    [9, 11],
                    [10, 11],
                    [12, 11],
                    [13, 11]
                ],
                "aaaaaaaaaaaaaa".to_string()
            ),
            13
        );
        assert_eq!(Solution::max_len(3, vec_vec![[0, 1], [1, 2]], "aba".to_string()), 3);
        assert_eq!(Solution::max_len(3, vec_vec![[0, 1], [0, 2]], "abc".to_string()), 1);
        assert_eq!(
            Solution::max_len(4, vec_vec![[0, 2], [0, 3], [3, 1]], "bbac".to_string()),
            3
        );
    }
}
