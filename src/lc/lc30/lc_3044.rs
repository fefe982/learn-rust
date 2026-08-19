// https://leetcode.com/problems/most-frequent-prime/
// 3044. Most Frequent Prime
pub struct Solution;
impl Solution {
    pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
        let mut prime_k = vec![true; 1001];
        let mut prime_v = vec![];
        for i in 2..1001 {
            if prime_k[i] {
                for j in (i * i..1001).step_by(i as usize) {
                    prime_k[j] = false;
                }
                prime_v.push(i as i32);
            }
        }
        let is_prime = |x: i32| -> bool {
            if x <= 1000 {
                return prime_k[x as usize];
            }
            for &p in &prime_v {
                if x % p == 0 {
                    return false;
                }
            }
            true
        };
        let mut prime_map = std::collections::HashMap::new();
        let mut max_c = 0;
        let mut max_p = -1;
        let mut push_number = |x: i32| {
            if !is_prime(x) {
                return;
            }
            let c = prime_map.entry(x).or_insert(0);
            *c += 1;
            if *c > max_c {
                max_c = *c;
                max_p = x;
            } else if *c == max_c {
                max_p = max_p.max(x);
            }
        };
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                for dir in [(-1, 0), (1, 0), (0, -1), (0, 1), (1, 1), (-1, -1), (1, -1), (-1, 1)] {
                    let mut n = mat[i][j];
                    let mut ii = i;
                    let mut jj = j;
                    loop {
                        ii = (ii as i32 + dir.0) as usize;
                        jj = (jj as i32 + dir.1) as usize;
                        if ii >= mat.len() || jj >= mat[ii].len() {
                            break;
                        }
                        n = n * 10 + mat[ii][jj];
                        push_number(n);
                    }
                }
            }
        }
        max_p
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn most_frequent_prime() {
        assert_eq!(Solution::most_frequent_prime(vec_vec![[1, 1], [9, 9], [1, 1]]), 19);
        assert_eq!(Solution::most_frequent_prime(vec_vec![[7]]), -1);
        assert_eq!(
            Solution::most_frequent_prime(vec_vec![[9, 7, 8], [4, 6, 5], [2, 8, 6]]),
            97
        );
    }
}
