// https://leetcode.com/problems/maximize-ysum-by-picking-a-triplet-of-distinct-xvalues/
// 3572. Maximize Y‑Sum by Picking a Triplet of Distinct X‑Values
pub struct Solution;
impl Solution {
    pub fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        for (x, y) in x.into_iter().zip(y.into_iter()) {
            if let Some(&yy) = m.get(&x) {
                if y <= yy {
                    continue;
                }
            }
            m.insert(x, y);
        }
        if m.len() < 3 {
            return -1;
        }
        let mut h = std::collections::BinaryHeap::new();
        for (_, y) in m.into_iter() {
            h.push(std::cmp::Reverse(y));
            if h.len() > 3 {
                h.pop();
            }
        }
        h.into_iter().map(|std::cmp::Reverse(y)| y).sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum_distinct_triplet() {
        assert_eq!(
            Solution::max_sum_distinct_triplet(vec![1, 2, 1, 3, 2], vec![5, 3, 4, 6, 2]),
            14
        );
        assert_eq!(
            Solution::max_sum_distinct_triplet(vec![1, 2, 1, 2], vec![4, 5, 6, 7]),
            -1
        );
    }
}
