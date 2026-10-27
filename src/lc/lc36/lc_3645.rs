// https://leetcode.com/problems/maximum-total-from-optimal-activation-order/
// 3645. Maximum Total from Optimal Activation Order
pub struct Solution;
impl Solution {
    pub fn max_total(value: Vec<i32>, limit: Vec<i32>) -> i64 {
        let mut m = std::collections::HashMap::new();
        for (&l, v) in limit.iter().zip(value.iter()) {
            let h = m.entry(l).or_insert(std::collections::BinaryHeap::new());
            h.push(std::cmp::Reverse(*v));
            if h.len() > l as usize {
                h.pop();
            }
        }
        m.iter()
            .map(|(_, h)| h.iter().map(|&std::cmp::Reverse(v)| v as i64).sum::<i64>())
            .sum::<i64>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_total() {
        assert_eq!(Solution::max_total(vec![3, 5, 8], vec![2, 1, 3]), 16);
        assert_eq!(Solution::max_total(vec![4, 2, 6], vec![1, 1, 1]), 6);
        assert_eq!(Solution::max_total(vec![4, 1, 5, 2], vec![3, 3, 2, 3]), 12);
    }
}
