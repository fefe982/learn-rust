// https://leetcode.com/problems/remove-stones-to-minimize-the-total/
// 1962. Remove Stones to Minimize the Total
pub struct Solution;
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        let mut sum = 0;
        for p in piles {
            q.push(p);
            sum += p;
        }
        for _ in 0..k {
            let n = q.pop().unwrap();
            sum -= n / 2;
            q.push(n - n / 2);
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_stone_sum() {
        assert_eq!(Solution::min_stone_sum(vec![5, 4, 9], 2), 12);
        assert_eq!(Solution::min_stone_sum(vec![4, 3, 6, 7], 3), 12);
    }
}
