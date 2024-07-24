// https://leetcode.com/problems/relocate-marbles/
// 2766. Relocate Marbles
pub struct Solution;
impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut pos = std::collections::BTreeSet::from_iter(nums.into_iter());
        for (f, t) in move_from.into_iter().zip(move_to.into_iter()) {
            if pos.remove(&f) {
                pos.insert(t);
            }
        }
        pos.into_iter().collect::<Vec<i32>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_relocate_marbles() {
        assert_eq!(
            Solution::relocate_marbles(vec![1, 6, 7, 8], vec![1, 7, 2], vec![2, 9, 5]),
            vec![5, 6, 8, 9]
        );
        assert_eq!(
            Solution::relocate_marbles(vec![1, 1, 3, 3], vec![1, 3], vec![2, 2]),
            vec![2]
        );
    }
}
