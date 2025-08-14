// https://leetcode.com/problems/last-stone-weight/
// 1046. Last Stone Weight
pub struct Solution;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = std::collections::binary_heap::BinaryHeap::from(stones);
        while stones.len() > 1 {
            let a = stones.pop().unwrap();
            let b = stones.pop().unwrap();
            if a > b {
                stones.push(a - b);
            }
        }
        if let Some(s) = stones.pop() {
            s
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_stone_weight() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }
}
