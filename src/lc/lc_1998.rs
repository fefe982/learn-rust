// https://leetcode.com/problems/gcd-sort-of-an-array/
// 1998. GCD Sort of an Array
pub struct Solution;
impl Solution {
    fn get_p(p: &mut Vec<usize>, i: usize) -> usize {
        let mut j = p[i];
        while j != p[j] {
            j = p[j];
        }
        p[i] = j;
        j
    }
    fn merge(p: &mut Vec<usize>, i: usize, j: usize) {
        let pi = Solution::get_p(p, i);
        let pj = Solution::get_p(p, j);
        if pi != pj {
            p[pi] = pj;
        }
    }
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        let mut p = vec![0; 100001];
        for &num in &nums {
            let mut n = num as usize;
            if p[n] != 0 {
                continue;
            }
            p[n] = n;
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    if p[i] == 0 {
                        p[i] = i;
                    }
                    Self::merge(&mut p, num as usize, i);
                    while n % i == 0 {
                        n /= i;
                    }
                }
                i += 1;
            }
            if n > 1 {
                if p[n] == 0 {
                    p[n] = n;
                }
                Self::merge(&mut p, num as usize, n);
            }
        }
        let mut nums_c = nums.clone();
        nums_c.sort();
        for (n, nc) in nums.into_iter().zip(nums_c.into_iter()) {
            if n != nc && Solution::get_p(&mut p, n as usize) != Solution::get_p(&mut p, nc as usize) {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd_sort() {
        assert_eq!(Solution::gcd_sort(vec![7, 21, 3]), true);
        assert_eq!(Solution::gcd_sort(vec![5, 2, 6, 2]), false);
        assert_eq!(Solution::gcd_sort(vec![10, 5, 9, 3, 15]), true);
    }
}
