// https://leetcode.com/problems/count-distinct-integers-after-removing-zeros/
// 3747. Count Distinct Integers After Removing Zeros
pub struct Solution;
impl Solution {
    pub fn count_distinct(n: i64) -> i64 {
        let mut v = vec![];
        let mut n = n;
        while n > 0 {
            v.push(n % 10);
            n /= 10;
        }
        for i in (0..v.len()).rev() {
            if v[i] == 0 {
                for j in i + 1..v.len() {
                    v[j] -= 1;
                    if v[j] > 0 {
                        break;
                    }
                    if j != v.len() - 1 {
                        v[j] = 9;
                    }
                }
                for j in 0..=i {
                    v[j] = 9;
                }
                break;
            }
        }
        let mut p9 = 1;
        let mut ans = 0;
        for i in 0..v.len() {
            if v[i] > 0 {
                ans += v[i] * p9;
                p9 *= 9;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_distinct() {
        assert_eq!(Solution::count_distinct(110), 90);
        assert_eq!(Solution::count_distinct(11), 10);
        assert_eq!(Solution::count_distinct(10), 9);
        assert_eq!(Solution::count_distinct(3), 3);
    }
}
