// https://leetcode.com/problems/find-the-maximum-number-of-elements-in-subset/
// 3020. Find the Maximum Number of Elements in a Subset
pub struct Solution;
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        for &num in &nums {
            *cnt.entry(num).or_insert(0) += 1;
        }
        let mut ans = 0;
        if let Some(v) = cnt.remove(&1) {
            if v % 2 == 0 {
                ans = v - 1;
            } else {
                ans = v;
            }
        }
        'n: for (&num, _) in cnt.iter() {
            let mut x = num;
            let mut r = 0;
            loop {
                let Some(&c) = cnt.get(&x) else {
                    break;
                };
                if c > 1 {
                    r += 2;
                } else {
                    break;
                }
                if let Some(y) = x.checked_mul(x) {
                    x = y;
                } else {
                    ans = ans.max(r - 1);
                    continue 'n;
                }
            }
            if cnt.contains_key(&x) {
                ans = ans.max(r + 1);
            } else {
                ans = ans.max(r - 1);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_length() {
        assert_eq!(Solution::maximum_length(vec![5, 4, 1, 2, 2]), 3);
        assert_eq!(Solution::maximum_length(vec![1, 3, 2, 4]), 1);
    }
}
