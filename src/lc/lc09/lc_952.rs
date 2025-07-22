// https://leetcode.com/problems/largest-component-size-by-common-factor/
// 952. Largest Component Size by Common Factor
pub struct Solution;
impl Solution {
    fn getp(p: &mut Vec<(usize, i32)>, x: usize) -> usize {
        if p[x].0 == x {
            return x;
        }
        p[x].0 = Self::getp(p, p[x].0);
        p[x].0
    }
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
            107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223,
            227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
        ];
        let mut parr = (0..50000).map(|i| (i, 0)).collect::<Vec<_>>();
        let mut ans = 1;
        for i in 0..nums.len() {
            let mut n = nums[i];
            if n == 1 {
                continue;
            }
            let mut p = usize::MAX;
            let mut sum = 0;
            for &f in &primes {
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
                    let pi = factor as usize;
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
