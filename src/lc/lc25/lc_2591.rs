// https://leetcode.com/problems/distribute-money-to-maximum-children/
// 2591. Distribute Money to Maximum Children
pub struct Solution;
impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money == children * 8 {
            return children;
        }
        if money / children >= 8 {
            return children - 1;
        }
        if money < children {
            return -1;
        }
        if money < children + 7 {
            return 0;
        }
        let money = money - children;
        let n = money / 7;
        let left = money % 7;
        if left != 3 {
            return n;
        }
        if n <= children - 2 {
            return n;
        }
        n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dist_money() {
        assert_eq!(Solution::dist_money(20, 3), 1);
        assert_eq!(Solution::dist_money(16, 2), 2);
    }
}
