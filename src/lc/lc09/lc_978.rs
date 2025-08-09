// https://leetcode.com/problems/longest-turbulent-subarray/
// 978. Longest Turbulent Subarray
pub struct Solution;
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut l = 0;
        let mut c = 1;
        for i in 1..arr.len() {
            match arr[i - 1].cmp(&arr[i]) {
                std::cmp::Ordering::Less => {
                    if l == 1 {
                        c += 1;
                    } else {
                        c = 2;
                    }
                    l = -1;
                }
                std::cmp::Ordering::Greater => {
                    if l == -1 {
                        c += 1;
                    } else {
                        c = 2;
                    }
                    l = 1;
                }
                std::cmp::Ordering::Equal => {
                    l = 0;
                    c = 1;
                }
            }
            res = res.max(c);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_turbulence_size() {
        assert_eq!(Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
        assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
        assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
    }
}
