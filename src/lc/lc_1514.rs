// https://leetcode.com/problems/path-with-maximum-probability/
// 1514. Path with Maximum Probability
pub struct Solution;
#[derive(PartialEq, PartialOrd)]
struct F64(f64);
impl std::cmp::Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl std::cmp::Eq for F64 {}
impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
        let mut dist = std::collections::HashMap::<i32, std::collections::HashMap<i32, f64>>::new();
        for (edge, prob) in edges.into_iter().zip(succ_prob.into_iter()) {
            dist.entry(edge[0]).or_default().insert(edge[1], prob);
            dist.entry(edge[1]).or_default().insert(edge[0], prob);
        }
        let mut visit = vec![false; n as usize];
        let mut q = std::collections::BinaryHeap::new();
        q.push((F64(1.0), start));
        while let Some((d, i)) = q.pop() {
            if i == end {
                return d.0;
            }
            if visit[i as usize] {
                continue;
            }
            visit[i as usize] = true;
            if let Some(disti) = dist.get(&i) {
                for (&to, &p) in disti {
                    if visit[to as usize] {
                        continue;
                    }
                    q.push((F64(d.0 * p), to));
                }
            }
        }
        0.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_probability() {
        assert_eq!(
            Solution::max_probability(
                500,
                vec_vec![[193, 229], [133, 212], [224, 465]],
                vec![0.5, 0.5, 0.2],
                4,
                364
            ),
            0.0
        );
        assert_eq!(
            Solution::max_probability(3, vec_vec![[0, 1], [1, 2], [0, 2]], vec![0.5, 0.5, 0.2], 0, 2),
            0.25
        );
        assert_eq!(
            Solution::max_probability(3, vec_vec![[0, 1], [1, 2], [0, 2]], vec![0.5, 0.5, 0.3], 0, 2),
            0.3
        );
        assert_eq!(Solution::max_probability(3, vec_vec![[0, 1]], vec![0.5], 0, 2), 0.0);
    }
}
