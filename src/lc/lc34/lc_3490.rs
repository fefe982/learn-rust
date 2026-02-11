// https://leetcode.com/problems/count-beautiful-numbers/
// 3490. Count Beautiful Numbers
pub struct Solution;
impl Solution {
    fn count(
        l: &Vec<i32>,
        r: &Vec<i32>,
        i: usize,
        sum: i32,
        prod: i32,
        limit_low: bool,
        limit_high: bool,
        cache: &mut std::collections::HashMap<(usize, i32, i32, bool, bool), i32>,
    ) -> i32 {
        if i == 0 {
            if prod % sum == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        let i = i - 1;
        let key = (i, sum, prod, limit_low, limit_high);
        if let Some(&v) = cache.get(&key) {
            return v;
        }
        let mut ans = 0;
        let mut dl = if limit_low { l[i] } else { 0 };
        if dl == -1 {
            ans += Self::count(l, r, i, 0, 1, true, false, cache);
            dl = 1;
        }
        let dh = if limit_high { r[i] } else { 9 };
        for k in dl..=dh {
            ans += Self::count(
                l,
                r,
                i,
                sum + k,
                prod * k,
                limit_low && k == l[i],
                limit_high && k == r[i],
                cache,
            );
        }
        cache.insert(key, ans);
        ans
    }
    pub fn beautiful_numbers(l: i32, r: i32) -> i32 {
        let mut l = l
            .to_string()
            .as_bytes()
            .iter()
            .rev()
            .map(|&c| (c - b'0') as i32)
            .collect::<Vec<_>>();
        let r = r
            .to_string()
            .as_bytes()
            .iter()
            .rev()
            .map(|&c| (c - b'0') as i32)
            .collect::<Vec<_>>();
        while l.len() < r.len() {
            l.push(-1);
        }
        Self::count(&l, &r, l.len(), 0, 1, true, true, &mut std::collections::HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn beautiful_numbers() {
        assert_eq!(Solution::beautiful_numbers(10, 20), 2);
        assert_eq!(Solution::beautiful_numbers(1, 15), 10);
    }
}
