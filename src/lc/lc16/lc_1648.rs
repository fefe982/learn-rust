// https://leetcode.com/problems/sell-diminishing-valued-colored-balls/
// 1648. Sell Diminishing-Valued Colored Balls
pub struct Solution;
impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut total = 0;
        let mut inventory = inventory;
        inventory.sort_unstable();
        let mut cnt = 1;
        let mut val = inventory.pop().unwrap();
        let mut orders = orders;
        while orders > 0 {
            let next_val = inventory.last().and_then(|&x| Some(x)).unwrap_or(0);
            let n = (orders / cnt).min(val - next_val);
            total += (val + val - n + 1) as i64 * n as i64 / 2 * cnt as i64;
            orders -= n * cnt;
            val = val - n;
            if val > next_val {
                total += val as i64 * orders as i64;
                break;
            }
            inventory.pop();
            cnt += 1;
        }
        (total % 1000_000_007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![2, 5], 4), 14);
        assert_eq!(Solution::max_profit(vec![3, 5], 6), 19);
    }
}
