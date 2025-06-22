// https://leetcode.com/problems/super-ugly-number/
// 313. Super Ugly Number
pub struct Solution;
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        q.push(std::cmp::Reverse(1));
        for _ in 1..n {
            let x: i32 = q.pop().unwrap().0;
            for &p in primes.iter() {
                if let Some(nx) = x.checked_mul(p) {
                    q.push(std::cmp::Reverse(nx));
                    if x % p == 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        q.pop().unwrap().0 as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nth_super_ugly_number() {
        assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
        assert_eq!(Solution::nth_super_ugly_number(1, vec![2, 3, 5]), 1);
    }
}
