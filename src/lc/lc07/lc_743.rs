// https://leetcode.com/problems/network-delay-time/
// 743. Network Delay Time
pub struct Solution;
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for t in times {
            g[t[0] as usize - 1].push((t[1] as usize - 1, t[2]));
        }
        let k = k as usize - 1;
        let mut cnt = 0;
        let mut q = std::collections::BinaryHeap::new();
        q.push(std::cmp::Reverse((0, k)));
        let mut v = vec![false; n as usize];
        let mut max = 0;
        while let Some(std::cmp::Reverse((d, i))) = q.pop() {
            if v[i] {
                continue;
            }
            max = d;
            v[i] = true;
            cnt += 1;
            for &(j, w) in g[i].iter() {
                if !v[j] {
                    q.push(std::cmp::Reverse((d + w, j)));
                }
            }
        }
        if cnt == n {
            max
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_network_delay_time() {
        assert_eq!(
            Solution::network_delay_time(vec_vec![[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2),
            2
        );
        assert_eq!(Solution::network_delay_time(vec_vec![[1, 2, 1]], 2, 1), 1);
        assert_eq!(Solution::network_delay_time(vec_vec![[1, 2, 1]], 2, 2), -1);
    }
}
