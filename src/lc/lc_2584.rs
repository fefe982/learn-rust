// https://leetcode.com/problems/split-the-array-to-make-coprime-products/
// 2584. Split the Array to Make Coprime Products
pub struct Solution;
impl Solution {
    pub fn find_valid_split(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return -1;
        }
        if nums[0] == 1 {
            return 0;
        }
        let mut p = vec![0; 1000001];
        let getp = |p: &mut Vec<usize>, i: usize| {
            if p[i] == 0 {
                p[i] = i;
                return i;
            }
            let mut pi = p[i];
            while pi != p[pi] {
                pi = p[pi];
            }
            p[i] = pi;
            pi
        };
        let merge = |p: &mut Vec<usize>, i: usize, j: usize| {
            let pi = getp(p, i);
            let pj = getp(p, j);
            if pi != pj {
                p[pj] = pi;
            }
        };
        for &n in &nums {
            let n = n as usize;
            if n == 1 || p[n] != 0 {
                continue;
            }
            let mut m = n;
            let mut i = 2;
            while m > 1 && i * i <= m {
                if m % i == 0 {
                    merge(&mut p, n, i);
                    while m % i == 0 {
                        m /= i;
                    }
                }
                i += 1;
            }
            if m > 1 {
                merge(&mut p, n, m);
            }
        }
        let mut p0 = usize::MAX;
        let mut p1 = usize::MAX;
        let mut split = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                if split == 0 {
                    split = i;
                }
                continue;
            }
            let pi = getp(&mut p, nums[i] as usize);
            if p0 == usize::MAX {
                p0 = pi;
            } else if p0 == pi {
                if p1 != usize::MAX {
                    p[p1] = p0;
                }
                p1 = usize::MAX;
                split = 0;
            } else {
                if p1 == usize::MAX {
                    p1 = pi;
                    if split == 0 {
                        split = i;
                    }
                } else if p1 != pi {
                    p[pi] = p1;
                }
            }
        }
        split as i32 - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_valid_split() {
        assert_eq!(Solution::find_valid_split(vec![1]), -1);
        assert_eq!(Solution::find_valid_split(vec![1, 41, 99, 17, 9, 8, 36, 70]), 0);
        assert_eq!(Solution::find_valid_split(vec![4, 7, 8, 15, 3, 5]), 2);
        assert_eq!(Solution::find_valid_split(vec![4, 7, 15, 8, 3, 5]), -1);
    }
}
