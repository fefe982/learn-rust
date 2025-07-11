// https://leetcode.com/problems/most-profitable-path-in-a-tree
// 2467. Most Profitable Path in a Tree
pub struct Solution;
impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        let gsize = edges.len() + 1;
        let mut graph = vec![vec![]; gsize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        let mut pv = vec![(usize::MAX, -1); gsize];
        let bob = bob as usize;
        pv[bob] = (usize::MAX, 0);
        let mut q = std::collections::VecDeque::new();
        q.push_back((bob, 0, usize::MAX));
        'z: while let Some((cur, cur_amount, prev)) = q.pop_front() {
            for &next in &graph[cur] {
                if next != prev {
                    pv[next] = (cur, cur_amount + 1);
                    q.push_back((next, cur_amount + 1, cur));
                    if next == 0 {
                        break 'z;
                    }
                }
            }
        }
        let len = pv[0].1;
        let hlen = if len % 2 == 0 { len / 2 } else { -1 };
        let mut bobn = 0;
        while bobn != usize::MAX {
            if pv[bobn].1 == hlen {
                amount[bobn] /= 2;
            } else if pv[bobn].1 <= len / 2 {
                amount[bobn] = 0;
            }
            bobn = pv[bobn].0;
        }
        q.clear();
        q.push_back((0, amount[0], usize::MAX));
        let mut max = i32::MIN;
        while let Some((cur, cur_amount, prev)) = q.pop_front() {
            if cur != 0 && graph[cur].len() == 1 {
                max = max.max(cur_amount);
            } else {
                for &next in &graph[cur] {
                    if next != prev {
                        q.push_back((next, cur_amount + amount[next], cur));
                    }
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_most_profitable_path() {
        assert_eq!(
            Solution::most_profitable_path(vec_vec![[0, 1], [1, 2], [1, 3], [3, 4]], 3, vec![-2, 4, 2, -4, 6]),
            6
        );
        assert_eq!(
            Solution::most_profitable_path(vec_vec![[0, 1]], 1, vec![-7280, 2350]),
            -7280
        );
    }
}
