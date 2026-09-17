// https://leetcode.com/problems/count-special-subsequences/
// 3404. Number of Special Subsequences
pub struct Solution;
impl Solution {
    pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        fn simplify(a: i32, b: i32) -> (i32, i32) {
            let g = gcd(a, b);
            (a / g, b / g)
        }
        let mut cnt = std::collections::HashMap::new();
        let mut ans = 0;
        for i in 4..nums.len() - 2 {
            let b = nums[i - 2];
            for &a in &nums[0..i - 3] {
                let (a, b) = simplify(a, b);
                *cnt.entry((a, b)).or_insert(0) += 1;
            }
            let c = nums[i];
            for &d in &nums[i + 2..] {
                let (c, d) = simplify(c, d);
                ans += *cnt.get(&(d, c)).unwrap_or(&0) as i64;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_subsequences() {
        assert_eq!(Solution::number_of_subsequences(vec![1, 2, 3, 4, 3, 6, 1]), 1);
        assert_eq!(Solution::number_of_subsequences(vec![3, 4, 3, 4, 3, 4, 3, 4]), 3);
    }
}
