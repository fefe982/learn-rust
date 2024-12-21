// https://leetcode.com/problems/sort-integers-by-the-power-value/
// 1387. Sort Integers by The Power Value
pub struct Solution;
impl Solution {
    fn getk(x: i32, cache: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if x == 1 {
            return 0;
        }
        if let Some(&v) = cache.get(&x) {
            return v;
        }
        let v = if x & 1 == 0 {
            Self::getk(x / 2, cache)
        } else {
            Self::getk(3 * x + 1, cache)
        } + 1;
        cache.insert(x, v);
        v
    }
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut cache = std::collections::HashMap::new();
        let mut v = (lo..=hi).map(|x| (Self::getk(x, &mut cache), x)).collect::<Vec<_>>();
        v.sort();
        v[k as usize - 1].1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_kth() {
        assert_eq!(Solution::get_kth(12, 15, 2), 13);
        assert_eq!(Solution::get_kth(7, 11, 4), 7);
    }
}
