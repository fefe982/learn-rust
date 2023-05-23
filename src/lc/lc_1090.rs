// https://leetcode.com/problems/largest-values-from-labels/
// 1090. Largest Values From Labels
pub struct Solution;
impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut ql = std::collections::HashMap::<i32, std::collections::BinaryHeap<i32>>::new();
        let use_limit = use_limit as usize;
        for (&v, &l) in values.iter().zip(labels.iter()) {
            let q = ql.entry(l).or_default();
            if q.len() == use_limit && v > -*q.peek().unwrap() {
                q.pop();
            }
            if q.len() < use_limit {
                q.push(-v);
            }
        }
        let mut qn = std::collections::BinaryHeap::<i32>::new();
        let num_wanted = num_wanted as usize;
        for (_, mut q) in ql {
            while let Some(n) = q.pop() {
                if qn.len() == num_wanted && n < *qn.peek().unwrap() {
                    qn.pop();
                }
                if qn.len() < num_wanted {
                    qn.push(n);
                }
            }
        }
        -qn.into_iter().sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_vals_from_labels() {
        assert_eq!(
            Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1),
            9
        );
        assert_eq!(
            Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2),
            12
        );
        assert_eq!(
            Solution::largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 1),
            16
        );
    }
}
