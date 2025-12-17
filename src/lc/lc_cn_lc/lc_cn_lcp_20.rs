// https://leetcode.cn/problems/meChtZ/
// LCP 20. 快速公交
pub struct Solution;
impl Solution {
    fn transit(
        target: i64,
        inc: i64,
        dec: i64,
        jump: &Vec<i32>,
        cost: &Vec<i32>,
        cache: &mut std::collections::HashMap<i64, i64>,
    ) -> i64 {
        if target == 0 {
            return 0;
        }
        if let Some(&v) = cache.get(&target) {
            return v;
        }
        let mut ans = target * inc;
        for i in 0..jump.len() {
            let j = jump[i] as i64;
            let c = cost[i] as i64;
            let r = target % j;
            if target > r {
                ans = ans.min(Self::transit(target / j, inc, dec, jump, cost, cache) + c + r * inc);
            }
            if r > 0 && target > 1 {
                ans = ans.min(Self::transit(target / j + 1, inc, dec, jump, cost, cache) + c + (j - r) * dec);
            }
        }
        cache.insert(target, ans);
        ans
    }
    pub fn bus_rapid_transit(target: i32, inc: i32, dec: i32, jump: Vec<i32>, cost: Vec<i32>) -> i32 {
        (Self::transit(
            target as i64,
            inc as i64,
            dec as i64,
            &jump,
            &cost,
            &mut std::collections::HashMap::new(),
        ) % 1_000_000_007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::bus_rapid_transit(31, 5, 3, vec![6], vec![10]), 33);
        assert_eq!(
            Solution::bus_rapid_transit(612, 4, 5, vec![3, 6, 8, 11, 5, 10, 4], vec![4, 7, 6, 3, 7, 6, 4]),
            26
        );
    }
}
