// https://leetcode.com/problems/largest-component-size-by-common-factor/
// 952. Largest Component Size by Common Factor
pub struct Solution;
impl Solution {
    fn seive(n: i32) -> std::collections::BTreeMap<i32, usize> {
        let mut v = vec![false; n as usize + 1];
        let mut i = 2;
        let mut res = std::collections::BTreeMap::<i32, usize>::new();
        let mut idx = 0usize;
        while i * i <= n {
            res.insert(i, idx);
            idx += 1;
            for j in 2.. {
                if i * j > n {
                    break;
                }
                v[(i * j) as usize] = true;
            }
            i += 1;
            while i < n && v[i as usize] {
                i += 1;
            }
        }
        while i < n {
            res.insert(i, idx);
            idx += 1;
            i += 1;
            while i < n && v[i as usize] {
                i += 1;
            }
        }
        res
    }
    fn getp(p: &mut Vec<(usize, i32)>, x: usize) -> usize {
        if p[x].0 == x {
            return x;
        }
        p[x].0 = Self::getp(p, p[x].0);
        p[x].0
    }
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let primes = Self::seive(50000);
        let mut parr = (0..primes.len()).map(|i| (i, 0)).collect::<Vec<_>>();
        let mut ans = 1;
        for i in 0..nums.len() {
            let mut n = nums[i];
            if n == 1 {
                continue;
            }
            let mut p = usize::MAX;
            let mut sum = 0;
            for &f in primes.keys() {
                let factor = if n % f == 0 {
                    f
                } else if f * f > n {
                    n
                } else {
                    -1
                };
                if factor > 50000 {
                    break;
                }
                if factor > 0 {
                    n /= factor;
                    while n % factor == 0 {
                        n /= factor;
                    }
                    let &pi = primes.get(&factor).unwrap();
                    let pp = Self::getp(&mut parr, pi);
                    if p == usize::MAX {
                        p = pp;
                        sum = parr[p].1;
                    } else {
                        if parr[pp].0 != p {
                            parr[pp].0 = p;
                            sum += parr[pp].1;
                        }
                    }
                }
                if n == 1 {
                    if p != usize::MAX {
                        parr[p].1 = sum + 1;
                        ans = ans.max(sum + 1);
                    }
                    break;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_component_size() {
        assert_eq!(Solution::largest_component_size(vec![4, 6, 15, 35]), 4);
        assert_eq!(Solution::largest_component_size(vec![20, 50, 9, 63]), 2);
        assert_eq!(Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39]), 8)
    }
}
