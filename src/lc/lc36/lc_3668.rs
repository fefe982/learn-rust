// https://leetcode.com/problems/restore-finishing-order/
// 3668. Restore Finishing Order
pub struct Solution;
impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let friends = friends.into_iter().collect::<std::collections::HashSet<_>>();
        order.into_iter().filter(|x| friends.contains(x)).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recover_order() {
        assert_eq!(
            Solution::recover_order(vec![3, 1, 2, 5, 4], vec![1, 3, 4]),
            vec![3, 1, 4]
        );
        assert_eq!(Solution::recover_order(vec![1, 4, 5, 3, 2], vec![2, 5]), vec![5, 2]);
    }
}
