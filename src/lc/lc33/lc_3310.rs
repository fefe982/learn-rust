// https://leetcode.com/problems/remove-methods-from-project/
// 3310. Remove Methods From Project
pub struct Solution;
impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut nz = 0;
        let mut nr = 0;
        let mut sus = vec![false; n];
        let mut g = vec![vec![]; n];
        let mut degree = vec![0; n];
        for i in 0..invocations.len() {
            let (a, b) = (invocations[i][0] as usize, invocations[i][1] as usize);
            g[a].push(b);
            degree[b] += 1;
        }
        let mut q = std::collections::VecDeque::new();
        let k = k as usize;
        if degree[k] == 0 {
            nz += 1;
        }
        q.push_back(k);
        nr += 1;
        sus[k] = true;
        while let Some(x) = q.pop_front() {
            for &y in &g[x] {
                if !sus[y] {
                    sus[y] = true;
                    q.push_back(y);
                    nr += 1;
                }
                degree[y] -= 1;
                if degree[y] == 0 {
                    nz += 1;
                }
            }
        }
        if nr == nz {
            sus.into_iter()
                .zip(0..)
                .filter_map(|(s, i)| if s { None } else { Some(i as i32) })
                .collect()
        } else {
            (0..n as i32).collect()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn remaining_methods() {
        assert_eq!(
            Solution::remaining_methods(4, 1, vec_vec![[1, 2], [0, 1], [3, 2]]),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            Solution::remaining_methods(5, 0, vec_vec![[1, 2], [0, 2], [0, 1], [3, 4]]),
            vec![3, 4]
        );
        assert_eq!(
            Solution::remaining_methods(3, 2, vec_vec![[1, 2], [0, 1], [2, 0]]),
            vec![]
        );
    }
}
