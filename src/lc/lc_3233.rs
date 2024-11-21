// https://leetcode.com/problems/find-the-count-of-numbers-which-are-not-special/
// 3233. Find the Count of Numbers Which Are Not Special
pub struct Solution;
impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let n = (r as f64).sqrt() as usize;
        let mut p = vec![true; n + 1];
        for i in 2..=(n as f64).sqrt() as usize {
            if p[i] {
                for j in 2..=n / i {
                    p[i * j] = false;
                }
            }
        }
        let p = p
            .into_iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i, v)| if v { Some((i * i) as i32) } else { None })
            .collect::<Vec<_>>();
        let ir = p.partition_point(|&x| x <= r);
        let il = p.partition_point(|&x| x < l);
        (r - l + 1) - (ir - il) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_non_special_count() {
        assert_eq!(Solution::non_special_count(5, 7), 3);
        assert_eq!(Solution::non_special_count(4, 16), 11);
    }
}
