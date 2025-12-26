// https://leetcode.com/problems/maximum-spending-after-buying-items/
// 2931. Maximum Spending After Buying Items
pub struct Solution;
impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let mut v = values.into_iter().flatten().collect::<Vec<_>>();
        v.sort_unstable();
        v.into_iter()
            .enumerate()
            .fold(0, |acc, (i, v)| acc + v as i64 * (i as i64 + 1))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_spending() {
        assert_eq!(Solution::max_spending(vec_vec![[8, 5, 2], [6, 4, 1], [9, 7, 3]]), 285);
        assert_eq!(Solution::max_spending(vec_vec![[10, 8, 6, 4, 2], [9, 7, 5, 3, 2]]), 386);
    }
}
