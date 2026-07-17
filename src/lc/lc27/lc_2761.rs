// https://leetcode.com/problems/prime-pairs-with-target-sum/
// 2761. Prime Pairs With Target Sum
pub struct Solution;
impl Solution {
    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        if n < 4 {
            return vec![];
        }
        if n == 4 {
            return vec![vec![2, 2]];
        }
        if n % 2 == 1 {
            let m = n - 2;
            let mut i = 3;
            while i * i <= m {
                if m % i == 0 {
                    return vec![];
                }
                i += 2;
            }
            return vec![vec![2, m]];
        }
        let mut prime = vec![true; n as usize];
        let mut i = 2;
        while i * i < n {
            if prime[i as usize] {
                let mut j = i * i;
                while j < n {
                    prime[j as usize] = false;
                    j += i;
                }
            }
            i += 1;
        }
        let mut res = vec![];
        for i in (3..=n / 2).step_by(2) {
            if prime[i as usize] && prime[(n - i) as usize] {
                res.push(vec![i, n - i])
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_prime_pairs() {
        assert_eq!(Solution::find_prime_pairs(5), vec_vec![[2, 3]]);
        assert_eq!(Solution::find_prime_pairs(10), vec_vec![[3, 7], [5, 5]]);
        assert_eq!(Solution::find_prime_pairs(2), Vec::<Vec<i32>>::new());
    }
}
