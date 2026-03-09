// https://leetcode.com/problems/count-good-meals/
// 1711. Count Good Meals
pub struct Solution;
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut m = std::collections::BTreeMap::new();
        for &d in deliciousness.iter() {
            *m.entry(d).or_insert(0) += 1;
        }
        let mut ans = 0;
        for (&k, &v) in m.iter().rev() {
            if k == 0 {
                continue;
            }
            let sub = (1 << (i32::BITS - k.leading_zeros())) - k;
            if sub < k {
                ans += v as i64 * m.get(&sub).cloned().unwrap_or(0) as i64;
            } else if sub == k {
                ans += v as i64 * (v - 1) as i64 / 2;
                ans += v as i64 * m.get(&0).cloned().unwrap_or(0) as i64;
            }
        }
        (ans % 1_000_000_007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_pairs() {
        assert_eq!(
            Solution::count_pairs(vec![
                149, 107, 1, 63, 0, 1, 6867, 1325, 5611, 2581, 39, 89, 46, 18, 12, 20, 22, 234
            ]),
            12
        );
        assert_eq!(Solution::count_pairs(vec![1, 3, 5, 7, 9]), 4);
        assert_eq!(Solution::count_pairs(vec![1, 1, 1, 3, 3, 3, 7]), 15);
    }
}
