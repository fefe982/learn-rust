// https://leetcode.com/problems/minimum-cost-to-hire-k-workers/
// 857. Minimum Cost to Hire K Workers
pub struct Solution;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut workers = quality.into_iter().zip(wage.into_iter()).collect::<Vec<_>>();
        workers.sort_by(|&(q1, w1), &(q2, w2)| (w1 as f64 / q1 as f64).partial_cmp(&(w2 as f64 / q2 as f64)).unwrap());
        let mut q = std::collections::BinaryHeap::new();
        let mut sum_q = 0;
        let mut ratio;
        for i in 0..k as usize {
            sum_q += workers[i].0;
            q.push(workers[i].0);
        }
        ratio = workers[k as usize - 1].1 as f64 / workers[k as usize - 1].0 as f64;
        let mut min = sum_q as f64 * ratio;
        for i in k as usize..workers.len() {
            sum_q += workers[i].0;
            sum_q -= q.pop().unwrap();
            q.push(workers[i].0);
            ratio = workers[i].1 as f64 / workers[i].0 as f64;
            min = min.min(sum_q as f64 * ratio);
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_mincost_to_hire_workers() {
        assert_approx_eq!(
            Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2),
            105.0,
            1e-5
        );
        assert_approx_eq!(
            Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3),
            30.66667,
            1e-5
        );
    }
}
