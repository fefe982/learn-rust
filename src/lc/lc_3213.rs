// https://leetcode.com/problems/construct-string-with-minimum-cost/
// 3213. Construct String With Minimum Cost
pub struct Solution;
impl Solution {
    pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        let mut wmap = vec![std::collections::HashMap::<&[u8], i32>::new(); 26];
        for (w, c) in words.iter().zip(costs) {
            let w = w.as_bytes();
            let idx = (w[0] - b'a') as usize;
            let entry = wmap[idx].entry(w).or_insert(c);
            *entry = (*entry).min(c);
        }
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(0), 0));
        let mut visited = vec![false; target.len()];
        let mut cost = vec![i32::MAX; target.len() + 1];
        cost[0] = 0;
        let target = target.as_bytes();
        while let Some((std::cmp::Reverse(c), l)) = q.pop() {
            if l == target.len() {
                return c;
            }
            if visited[l] {
                continue;
            }
            visited[l] = true;
            let idx = (target[l] - b'a') as usize;
            for (&w, &c) in wmap[idx].iter() {
                if w.len() + l <= target.len() && cost[w.len() + l] > cost[l] + c && &target[l..l + w.len()] == w {
                    cost[w.len() + l] = cost[l] + c;
                    q.push((std::cmp::Reverse(cost[w.len() + l]), w.len() + l));
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
    fn test_minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(
                "abcdef".to_string(),
                vec_str!["abdef", "abc", "d", "def", "ef"],
                vec![100, 1, 1, 10, 5]
            ),
            7
        );
        assert_eq!(
            Solution::minimum_cost("aaaa".to_string(), vec_str!["z", "zz", "zzz"], vec![1, 10, 100]),
            -1
        );
    }
}
