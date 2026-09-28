// https://leetcode.com/problems/eat-pizzas/
// 3457. Eat Pizzas!
pub struct Solution;
impl Solution {
    pub fn max_weight(pizzas: Vec<i32>) -> i64 {
        let mut pizzas = pizzas;
        pizzas.sort_unstable_by_key(|x| -x);
        let n = pizzas.len() / 4;
        let e = n / 2;
        let o = n - e;
        let mut sum = 0;
        for i in 0..o {
            sum += pizzas[i] as i64;
        }
        for i in 0..e {
            sum += pizzas[o + i * 2 + 1] as i64;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_weight() {
        assert_eq!(Solution::max_weight(vec![1, 2, 3, 4, 5, 6, 7, 8]), 14);
        assert_eq!(Solution::max_weight(vec![2, 1, 1, 1, 1, 1, 1, 1]), 3);
    }
}
