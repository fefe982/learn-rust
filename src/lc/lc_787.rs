// https://leetcode.com/problems/cheapest-flights-within-k-stops/
// 787. Cheapest Flights Within K Stops
pub struct Solution;
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut cost = vec![i32::MAX; n];
        let mut step = vec![-2; n];
        let src = src as usize;
        let dst = dst as usize;
        cost[src] = 0;
        step[src] = -1;
        let mut edge = vec![vec![]; n];
        for f in flights {
            edge[f[0] as usize].push((f[1] as usize, f[2]));
        }
        let mut q = std::collections::BinaryHeap::new();
        for e in &edge[src] {
            q.push(std::cmp::Reverse((e.1, 0, e.0)));
        }
        while let Some(std::cmp::Reverse((price, n, cur))) = q.pop() {
            if cur == dst {
                return price;
            }
            if cost[cur] == i32::MAX || n < step[cur] {
                cost[cur] = price;
                step[cur] = n;
                if n < k {
                    for e in &edge[cur] {
                        q.push(std::cmp::Reverse((price + e.1, n + 1, e.0)));
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_cheapest_price() {
        assert_eq!(
            Solution::find_cheapest_price(
                4,
                vec_vec![[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]],
                0,
                3,
                1
            ),
            700
        );
        assert_eq!(
            Solution::find_cheapest_price(3, vec_vec![[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 1),
            200
        );
        assert_eq!(
            Solution::find_cheapest_price(3, vec_vec![[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 0),
            500
        );
    }
}
