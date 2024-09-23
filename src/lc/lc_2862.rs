// https://leetcode.com/problems/maximum-element-sum-of-a-complete-subset-of-indices/
// 2862. Maximum Element Sum of a Complete Subset of Indices
pub struct Solution;
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let bound = n / 4;
        let getprimes = |n: usize| -> Vec<usize> {
            let mut flags = vec![true; n as usize + 1];
            let mut primes = vec![];
            let mut i = 2;
            while i * i <= n {
                if flags[i as usize] {
                    primes.push(i);
                    let mut j = i * i;
                    while j <= n {
                        flags[j as usize] = false;
                        j += i;
                    }
                }
                i += 1;
            }
            for j in i..=n {
                if flags[j as usize] {
                    primes.push(j);
                }
            }
            primes
        };
        let primes = getprimes(n);
        let mut q = vec![(1, 0)];
        let mut max = 0;
        while let Some((base, index)) = q.pop() {
            let mut sum = 0;
            for i in 1.. {
                if i * i * base <= n {
                    sum += nums[i * i * base - 1] as i64;
                } else {
                    break;
                }
            }
            max = max.max(sum);
            for i in index..primes.len() {
                if base * primes[i] <= bound {
                    q.push((base * primes[i], i + 1));
                }
            }
        }
        max.max(*nums.iter().max().unwrap() as i64)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_sum() {
        assert_eq!(Solution::maximum_sum(vec![8, 7, 3, 5, 7, 2, 4, 9]), 16);
        assert_eq!(Solution::maximum_sum(vec![8, 10, 3, 8, 1, 13, 7, 9, 4]), 20);
    }
}
