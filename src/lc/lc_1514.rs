// https://leetcode.com/problems/path-with-maximum-probability/
// 1514. Path with Maximum Probability
pub struct Solution;
impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut dist = std::collections::HashMap::<i32, std::collections::HashMap<i32, f64>>::new();
        for (edge, prob) in edges.into_iter().zip(succ_prob.into_iter()) {
            dist.entry(edge[0]).or_default().insert(edge[1], prob);
            dist.entry(edge[1]).or_default().insert(edge[0], prob);
        }
        let mut max_prob = vec![0.0; n as usize];
        max_prob[start as usize] = 1.0;
        let mut q = std::collections::VecDeque::new();
        q.push_back(start);
        while let Some(i) = q.pop_front() {
            let di = max_prob[i as usize];
            if let Some(disti) = dist.get(&i) {
                for (&to, &p) in disti {
                    let nmax = di * p;
                    if nmax > max_prob[to as usize] {
                        q.push_back(to);
                        max_prob[to as usize] = nmax;
                    }
                }
            }
        }
        max_prob[end as usize]
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
            Solution::max_probability(
                3,
                vec_vec![[0, 1], [1, 2], [0, 2]],
                vec![0.5, 0.5, 0.2],
                0,
                2
            ),
            0.25
        );
        assert_eq!(
            Solution::max_probability(
                3,
                vec_vec![[0, 1], [1, 2], [0, 2]],
                vec![0.5, 0.5, 0.3],
                0,
                2
            ),
            0.3
        );
        assert_eq!(
            Solution::max_probability(3, vec_vec![[0, 1]], vec![0.5], 0, 2),
            0.0
        );
    }
}
