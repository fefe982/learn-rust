// https://leetcode.com/problems/number-of-beautiful-integers-in-the-range/
// 2827. Number of Beautiful Integers in the Range
pub struct Solution;
impl Solution {
    fn count_inner(
        s: &[u8],
        idx: usize,
        bound: usize,
        even: usize,
        odd: usize,
        remainder: usize,
        k: usize,
        cache: &mut Vec<Vec<Vec<Vec<Vec<i32>>>>>,
    ) -> i32 {
        if idx >= s.len() {
            return if even == odd && remainder == 0 { 1 } else { 0 };
        }
        if even * 2 > s.len() || odd * 2 > s.len() {
            return 0;
        }
        if cache[idx][bound][even][odd][remainder] != -1 {
            return cache[idx][bound][even][odd][remainder];
        }
        let mut val = 0;
        if even == 0 && odd == 0 && (s.len() - idx) % 2 == 1 {
            val = Self::count_inner(s, idx + 1, 0, 0, 0, 0, k, cache);
        } else {
            let limit = if bound == 0 { 9 } else { (s[idx] - b'0') as usize };
            for digit in 0..=limit {
                if digit == 0 && even == 0 && odd == 0 {
                    val += Self::count_inner(s, idx + 2, 0, 0, 0, 0, k, cache);
                } else {
                    let o = digit % 2;
                    val += Self::count_inner(
                        s,
                        idx + 1,
                        if bound == 1 && digit == limit { 1 } else { 0 },
                        even + 1 - o,
                        odd + o,
                        (remainder * 10 + digit) % k,
                        k,
                        cache,
                    );
                }
            }
        }
        cache[idx][bound][even][odd][remainder] = val;
        val
    }
    fn count(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        Self::count_inner(
            s,
            0,
            1,
            0,
            0,
            0,
            k as usize,
            &mut vec![vec![vec![vec![vec![-1; k as usize]; len / 2 + 1]; len / 2 + 1]; 2]; len],
        )
    }
    pub fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
        Self::count(high.to_string(), k) - Self::count((low - 1).to_string(), k)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_beautiful_integers() {
        assert_eq!(Solution::number_of_beautiful_integers(10, 20, 3), 2);
        assert_eq!(Solution::number_of_beautiful_integers(1, 10, 1), 1);
        assert_eq!(Solution::number_of_beautiful_integers(5, 5, 2), 0);
    }
}
