// https://leetcode.com/problems/take-gifts-from-the-richest-pile/
// 2558. Take Gifts From the Richest Pile
pub struct Solution;
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut h: std::collections::BinaryHeap<i32> = gifts.into_iter().collect();
        for _ in 0..k {
            let n = h.pop().unwrap();
            h.push((n as f64).sqrt() as i32);
        }
        h.into_iter().map(|x| x as i64).sum::<i64>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pick_gifts() {
        assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
        assert_eq!(Solution::pick_gifts(vec![1, 1, 1, 1], 4), 4);
    }
}
