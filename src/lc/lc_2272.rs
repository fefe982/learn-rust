// https://leetcode.com/problems/substring-with-largest-variance/description/
// 2272. Substring With Largest Variance
pub struct Solution;
impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let mut max = 0;
        let s = s.as_bytes();
        for h in b'a'..=b'z' {
            for l in b'a'..=b'z' {
                if h == l {
                    continue;
                }
                let mut stat = (0, false, false);
                for &b in s {
                    if b != h && b != l {
                        continue;
                    }
                    if b == h {
                        stat = (stat.0 + 1, stat.1, stat.2);
                    } else {
                        stat = if stat.2 && stat.1 && stat.0 >= 0 {
                            (stat.0, true, false)
                        } else if stat.0 > 0 {
                            (stat.0 - 1, true, stat.2)
                        } else {
                            (-1, true, true)
                        };
                    }
                    if stat.1 {
                        max = max.max(stat.0);
                    }
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_variance() {
        assert_eq!(Solution::largest_variance(String::from("baacca")), 2);
        assert_eq!(Solution::largest_variance(String::from("ababab")), 1);
        assert_eq!(Solution::largest_variance(String::from("baaa")), 2);
        assert_eq!(Solution::largest_variance(String::from("lripaa")), 1);
        assert_eq!(Solution::largest_variance(String::from("aababbb")), 3);
        assert_eq!(Solution::largest_variance(String::from("abcde")), 0)
    }
}
