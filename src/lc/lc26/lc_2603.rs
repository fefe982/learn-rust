// https://leetcode.com/problems/collect-coins-in-a-tree/
// 2603. Collect Coins in a Tree
pub struct Solution;
impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = std::collections::HashMap::<usize, std::collections::HashSet<usize>>::new();
        for edge in edges {
            graph
                .entry(edge[0] as usize)
                .or_default()
                .insert(edge[1] as usize);
            graph
                .entry(edge[1] as usize)
                .or_default()
                .insert(edge[0] as usize);
        }
        let mut v_node: Vec<usize> = graph
            .iter()
            .filter_map(|(&i, x)| {
                if coins[i] == 0 && x.len() == 1 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        while let Some(n) = v_node.pop() {
            for &e in graph.remove(&n).iter().flatten() {
                if let Some(vv) = graph.get_mut(&e) {
                    vv.remove(&n);
                    if coins[e] == 0 && vv.len() == 1 {
                        v_node.push(e);
                    }
                }
            }
        }
        let mut v_coin: std::collections::VecDeque<(usize, i32)> = graph
            .iter()
            .filter_map(|(&i, x)| if x.len() == 1 { Some((i, 0)) } else { None })
            .collect();
        while let Some((n, c)) = v_coin.pop_front() {
            for &e in graph.remove(&n).iter().flatten() {
                if let Some(vv) = graph.get_mut(&e) {
                    vv.remove(&n);
                    if c < 1 && vv.len() == 1 {
                        v_coin.push_back((e, c + 1));
                    }
                }
            }
        }
        if graph.len() == 0 {
            0
        } else {
            2 * (graph.len() as i32 - 1)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_collect_the_coins() {
        assert_eq!(
            Solution::collect_the_coins(
                vec![1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1],
                vec_vec![
                    [0, 1],
                    [1, 2],
                    [2, 3],
                    [1, 4],
                    [4, 5],
                    [5, 6],
                    [6, 7],
                    [3, 8],
                    [6, 9],
                    [7, 10],
                    [10, 11],
                    [10, 12],
                    [7, 13],
                    [12, 14],
                    [13, 15],
                    [14, 16],
                    [15, 17],
                    [10, 18]
                ]
            ),
            12
        );
        assert_eq!(
            Solution::collect_the_coins(
                vec![1, 0, 0, 0, 0, 1],
                vec_vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]]
            ),
            2
        );
        assert_eq!(
            Solution::collect_the_coins(
                vec![0, 0, 0, 1, 1, 0, 0, 1],
                vec_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [5, 6], [5, 7]]
            ),
            2
        );
    }
}
