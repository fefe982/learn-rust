// https://leetcode.com/problems/minimum-edge-toggles-on-a-tree/
// 3812. Minimum Edge Toggles on a Tree
pub struct Solution;
impl Solution {
    pub fn minimum_flips(n: i32, edges: Vec<Vec<i32>>, start: String, target: String) -> Vec<i32> {
        let n = n as usize;
        let mut nc = 0;
        let mut change = start
            .as_bytes()
            .iter()
            .zip(target.as_bytes().iter())
            .map(|(s, t)| {
                if s != t {
                    nc += 1;
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        if nc == 0 {
            return vec![];
        }
        if nc % 2 == 1 {
            return vec![-1];
        }
        let mut g = vec![Some(std::collections::HashMap::new()); n];
        let mut d = vec![0; n];
        for (e, idx) in edges.into_iter().zip(0..) {
            (g[e[0] as usize]).as_mut().unwrap().insert(e[1] as usize, idx);
            g[e[1] as usize].as_mut().unwrap().insert(e[0] as usize, idx);
            d[e[0] as usize] += 1;
            d[e[1] as usize] += 1;
        }
        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            if d[i] == 1 {
                q.push_back(i);
            }
        }
        let mut ans = Vec::with_capacity(n);
        while let Some(u) = q.pop_front() {
            if d[u] == 0 {
                continue;
            }
            for (&v, &idx) in g[u].take().unwrap().iter() {
                d[v] -= 1;
                if change[u] {
                    ans.push(idx as i32);
                    change[v] = !change[v];
                }
                g[v].as_mut().unwrap().remove(&u);
                if d[v] == 1 {
                    q.push_back(v);
                }
            }
        }
        ans.sort();
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_flips() {
        assert_eq!(
            Solution::minimum_flips(3, vec_vec![[0, 1], [1, 2]], "010".to_string(), "100".to_string()),
            [0]
        );
        assert_eq!(
            Solution::minimum_flips(
                7,
                vec_vec![[0, 1], [1, 2], [2, 3], [3, 4], [3, 5], [1, 6]],
                "0011000".to_string(),
                "0010001".to_string()
            ),
            [1, 2, 5]
        );
    }
}
