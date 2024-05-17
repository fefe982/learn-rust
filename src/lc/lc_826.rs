// https://leetcode.com/problems/most-profit-assigning-work/
// 826. Most Profit Assigning Work
pub struct Solution;
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut m = std::collections::BTreeMap::<i32, i32>::new();
        for (d, p) in difficulty.into_iter().zip(profit.into_iter()) {
            if let Some((&_, &lp)) = m.range(..=d).next_back() {
                if lp < p {
                    m.insert(d, p);
                }
            } else {
                m.insert(d, p);
            }
            while let Some((&ld, &lp)) = m.range(d + 1..).next() {
                if lp <= p {
                    m.remove(&ld);
                } else {
                    break;
                }
            }
        }
        worker
            .into_iter()
            .fold(0, |acc, w| acc + m.range(..=w).next_back().map_or(0, |(_, &p)| p))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_profit_assignment() {
        assert_eq!(
            Solution::max_profit_assignment(vec![2, 4, 6, 8, 10], vec![10, 20, 30, 40, 50], vec![4, 5, 6, 7]),
            100
        );
        assert_eq!(
            Solution::max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25]),
            0
        );
    }
}
