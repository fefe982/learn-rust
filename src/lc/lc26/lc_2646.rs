// https://leetcode.com/problems/minimize-the-total-price-of-the-trips/
// 2646. Minimize the Total Price of the Trips
pub struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut reachable = vec![HashMap::<usize, HashSet<usize>>::new(); n];
        let mut route = vec![vec![usize::MAX; n]; n];
        let mut next_node = vec![vec![]; n];
        for edge in edges.iter() {
            next_node[edge[0] as usize].push(edge[1] as usize);
            next_node[edge[1] as usize].push(edge[0] as usize);
        }
        for i in 0..n {
            let mut q = vec![(usize::MAX, i, 0)];
            while let Some((p, node, visit)) = q.pop() {
                if visit == 0 {
                    q.push((p, node, 1));
                    for &next in &next_node[node] {
                        if next != p && !reachable[node].contains_key(&next) {
                            q.push((node, next, 0));
                        }
                    }
                } else {
                    if p != usize::MAX {
                        let mut set = HashSet::new();
                        for &next in &next_node[node] {
                            if next != p {
                                set = set.union(reachable[node].get(&next).unwrap()).cloned().collect();
                            }
                        }
                        set.insert(node);
                        reachable[p].insert(node, set);
                    }
                }
            }
        }
        for i in 0..n {
            route[i][i] = i;
            for (&next, reach) in &reachable[i] {
                for &r in reach {
                    route[i][r] = next;
                }
            }
        }
        reachable.clear();
        let mut cnt = vec![0; n];
        for trip in trips {
            let mut s = trip[0] as usize;
            let e = trip[1] as usize;
            while s != e {
                cnt[s] += 1;
                s = route[s][e];
            }
            cnt[e] += 1;
        }
        let mut q = vec![(usize::MAX, 0, 0)];
        let mut minprice = vec![vec![0; n]; 2];
        while let Some((p, node, visit)) = q.pop() {
            if visit == 0 {
                q.push((p, node, 1));
                for &next in &next_node[node] {
                    if next != p {
                        q.push((node, next, 0));
                    }
                }
            } else {
                let mut min_all = 0;
                let mut min_keep = 0;
                for &next in &next_node[node] {
                    if next != p {
                        min_all += minprice[0][next].min(minprice[1][next]);
                        min_keep += minprice[0][next];
                    }
                }
                minprice[0][node] = min_all + cnt[node] * price[node];
                minprice[1][node] = min_keep + cnt[node] * price[node] / 2;
            }
        }
        minprice[0][0].min(minprice[1][0])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_total_price() {
        assert_eq!(
            Solution::minimum_total_price(
                9,
                vec_vec![[2, 5], [3, 4], [4, 1], [1, 7], [6, 7], [7, 0], [0, 5], [5, 8]],
                vec![4, 4, 6, 4, 2, 4, 2, 14, 8],
                vec_vec![
                    [1, 5],
                    [2, 7],
                    [4, 3],
                    [1, 8],
                    [2, 8],
                    [4, 3],
                    [1, 5],
                    [1, 4],
                    [2, 1],
                    [6, 0],
                    [0, 7],
                    [8, 6],
                    [4, 0],
                    [7, 5],
                    [7, 5],
                    [6, 0],
                    [5, 1],
                    [1, 1],
                    [7, 5],
                    [1, 7],
                    [8, 7],
                    [2, 3],
                    [4, 1],
                    [3, 5],
                    [2, 5],
                    [3, 7],
                    [0, 1],
                    [5, 8],
                    [5, 3],
                    [5, 2]
                ]
            ),
            429
        );
        assert_eq!(
            Solution::minimum_total_price(
                4,
                vec_vec![[0, 1], [1, 2], [1, 3]],
                vec![2, 2, 10, 6],
                vec_vec![[0, 3], [2, 1], [2, 3]]
            ),
            23
        );
        assert_eq!(
            Solution::minimum_total_price(2, vec_vec![[0, 1]], vec![2, 2], vec_vec![[0, 0]]),
            1
        );
    }
}
