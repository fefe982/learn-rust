// https://leetcode.com/problems/count-prime-gap-balanced-subarrays/
// 3589. Count Prime-Gap Balanced Subarrays
pub struct Solution;
impl Solution {
    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as i32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    pub fn prime_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut start = 0;
        let mut plast = 0;
        let mut last = 0;
        let mut minh = std::collections::BinaryHeap::new();
        let mut maxh: std::collections::BinaryHeap<(i32, usize)> = std::collections::BinaryHeap::new();
        let mut ans = 0;
        nums.iter().enumerate().for_each(|(i, &x)| {
            if Self::is_prime(x) {
                if !minh.is_empty() {
                    if x > maxh.peek().unwrap().0 {
                        while let Some(&(std::cmp::Reverse(min), imin)) = minh.peek() {
                            if x - min > k {
                                minh.pop();
                                start = start.max(imin + 1);
                            } else {
                                break;
                            }
                        }
                    } else if x < minh.peek().unwrap().0 .0 {
                        while let Some(&(max, imax)) = maxh.peek() {
                            if max - x > k {
                                maxh.pop();
                                start = start.max(imax + 1);
                            } else {
                                break;
                            }
                        }
                    }
                }
                minh.push((std::cmp::Reverse(x), i));
                maxh.push((x, i));
                plast = last;
                last = i;
            }
            if minh.len() > 1 && maxh.len() > 1 {
                ans += (plast - start + 1) as i32;
            }
        });
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prime_subarray() {
        assert_eq!(Solution::prime_subarray(vec![1, 2, 3], 1), 2);
        assert_eq!(Solution::prime_subarray(vec![2, 3, 5, 7], 3), 4);
    }
}
