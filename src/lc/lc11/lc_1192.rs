// https://leetcode.com/problems/critical-connections-in-a-network/
// 1192. Critical Connections in a Network
pub struct Solution;
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut g = vec![vec![]; n as usize];
        for v in connections {
            g[v[0] as usize].push(v[1]);
            g[v[1] as usize].push(v[0]);
        }
        let mut parent = vec![-1; n as usize];
        let mut lp = vec![false; n as usize];
        let mut q = vec![0];
        while let Some(cur) = q.pop() {
            let mut path = std::collections::HashSet::<i32>::new();
            for &next in g[cur as usize].iter() {
                if next == parent[cur as usize] {
                    continue;
                }
                if next == 0 || parent[next as usize] >= 0 {
                    if path.is_empty() {
                        let mut pcur = cur;
                        while pcur != 0 {
                            path.insert(parent[pcur as usize]);
                            pcur = parent[pcur as usize];
                        }
                    }
                    let mut pnext = next;
                    while !path.contains(&pnext) {
                        lp[pnext as usize] = true;
                        pnext = parent[pnext as usize];
                    }
                    let mut pcur = cur;
                    while pcur != pnext {
                        lp[pcur as usize] = true;
                        pcur = parent[pcur as usize];
                    }
                } else {
                    parent[next as usize] = cur;
                    q.push(next);
                }
            }
        }
        parent
            .into_iter()
            .zip(lp.into_iter())
            .enumerate()
            .skip(1)
            .filter_map(|(i, (p, lp))| if !lp { Some(vec![p, i as i32]) } else { None })
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn mod_vec(v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut v = v
            .into_iter()
            .map(|mut x| {
                x.sort_unstable();
                x
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }
    #[test]
    fn test_critical_connections() {
        assert_eq!(
            mod_vec(Solution::critical_connections(
                4,
                vec_vec![[0, 1], [1, 2], [2, 0], [1, 3]]
            )),
            mod_vec(vec_vec![[1, 3]])
        );
        assert_eq!(
            mod_vec(Solution::critical_connections(2, vec_vec![[0, 1]])),
            mod_vec(vec_vec![[0, 1]])
        );
    }
}
