// https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair/
// 3480. Maximize Subarrays After Removing One Element
pub struct Solution;
impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let mut pair = conflicting_pairs
            .into_iter()
            .map(|e| (e[0].min(e[1]), e[0].max(e[1])))
            .collect::<Vec<_>>();
        pair.sort_by(|a, b| a.1.cmp(&b.1));
        let mut cnt = 0i64;
        // let mut h = std::collections::BinaryHeap::new();
        let mut p = 0;
        let mut max = 0;
        let mut maxs = 0;
        let mut lasts = 0;
        let mut add = 0i64;
        for e in 1..=n {
            while p < pair.len() && pair[p].1 <= e {
                if pair[p].0 >= maxs {
                    add = 0;
                    lasts = maxs;
                    maxs = pair[p].0;
                } else if pair[p].0 >= lasts {
                    lasts = pair[p].0;
                }
                p += 1;
            }
            add += (maxs - lasts) as i64;
            max = max.max(add);
            cnt += (e - maxs) as i64;
        }
        cnt + max as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_subarrays() {
        assert_eq!(
            Solution::max_subarrays(100000, vec_vec![[50000, 50001], [99999, 100000]]),
            4999950001
        );
        assert_eq!(Solution::max_subarrays(10, vec_vec![[10, 5], [3, 8]]), 50);
        assert_eq!(Solution::max_subarrays(4, vec_vec![[2, 3], [1, 4]]), 9);
        assert_eq!(Solution::max_subarrays(5, vec_vec![[1, 2], [2, 5], [3, 5]]), 12);
    }
}
