// https://leetcode.com/problems/count-square-sum-triples/
// 1925. Count Square Sum Triples
pub struct Solution;
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            loop {
                if b == 0 {
                    break a;
                }
                a = a % b;
                if a == 0 {
                    break b;
                }
                b = b % a;
            }
        }
        let mut ans = 0;
        let mut u = 3;
        while u * u <= n * 2 {
            let mut v = 1;
            while v < u && u * u + v * v <= n * 2 {
                if gcd(u, v) == 1 {
                    ans += n * 2 / (u * u + v * v);
                }
                v += 2;
            }
            u += 2;
        }
        ans * 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_triples() {
        assert_eq!(Solution::count_triples(5), 2);
        assert_eq!(Solution::count_triples(10), 4);
    }
}
