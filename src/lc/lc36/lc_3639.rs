// https://leetcode.com/problems/minimum-time-to-activate-string/
// 3639. Minimum Time to Activate a String
pub struct Solution;
impl Solution {
    pub fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
        let _ = s;
        let mut m = std::collections::BTreeSet::new();
        m.insert(-1);
        m.insert(order.len() as i32);
        let mut k = k;
        for (id, i) in order.into_iter().enumerate() {
            let l = *m.range(..i).last().unwrap();
            let r = *m.range(i..).next().unwrap();
            k -= (i - l) * (r - i);
            if k <= 0 {
                return id as i32;
            }
            m.insert(i);
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_time() {
        assert_eq!(Solution::min_time("abc".to_string(), vec![1, 0, 2], 2), 0);
        assert_eq!(Solution::min_time("cat".to_string(), vec![0, 2, 1], 6), 2);
        assert_eq!(Solution::min_time("xy".to_string(), vec![0, 1], 4), -1);
    }
}
