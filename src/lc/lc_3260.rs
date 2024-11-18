// https://leetcode.com/problems/find-the-largest-palindrome-divisible-by-k/
// 3260. Find the Largest Palindrome Divisible by K
pub struct Solution;
impl Solution {
    pub fn largest_palindrome(n: i32, k: i32) -> String {
        match k {
            1 | 3 | 9 => "9".repeat(n as usize),
            2 => {
                if n == 1 {
                    "8".to_string()
                } else {
                    "8".to_string() + &"9".repeat(n as usize - 2) + "8"
                }
            }
            4 => {
                if n < 5 {
                    "8".repeat(n as usize)
                } else {
                    "88".to_string() + &"9".repeat(n as usize - 4) + "88"
                }
            }
            5 => {
                if n == 1 {
                    "5".to_string()
                } else {
                    "5".to_string() + &"9".repeat(n as usize - 2) + "5"
                }
            }
            6 => {
                if n == 1 {
                    "6".to_string()
                } else if n == 2 {
                    "66".to_string()
                } else if n % 2 == 0 {
                    "8".to_string() + &"9".repeat(n as usize / 2 - 2) + "77" + &"9".repeat(n as usize / 2 - 2) + "8"
                } else {
                    "8".to_string() + &"9".repeat(n as usize / 2 - 1) + "8" + &"9".repeat(n as usize / 2 - 1) + "8"
                }
            }
            7 => {
                if n == 1 {
                    "7".to_string()
                } else if n == 2 {
                    "77".to_string()
                } else {
                    let mut pow = 1;
                    let mut part = 0;
                    for _ in 0..(n - 1) / 2 {
                        part = (part + 9 * pow) % 7;
                        pow = (pow * 10) % 7;
                    }
                    let mid = if n % 2 == 0 {
                        part = (part + part * pow * 100) % 7;
                        (pow * 11) % 7
                    } else {
                        part = (part + part * pow * 10) % 7;
                        pow
                    };
                    let mut midd = 0;
                    for j in (0..=9).rev() {
                        if (mid * j + part) % 7 == 0 {
                            midd = j;
                            break;
                        }
                    }
                    "9".repeat((n as usize - 1) / 2)
                        + &midd.to_string().repeat(2 - n as usize % 2)
                        + &"9".repeat((n as usize - 1) / 2)
                }
            }
            8 => {
                if n < 7 {
                    "8".repeat(n as usize)
                } else {
                    "888".to_string() + &"9".repeat(n as usize - 6) + "888"
                }
            }
            _ => unreachable!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_palindrome() {
        assert_eq!(Solution::largest_palindrome(3, 5), "595");
        assert_eq!(Solution::largest_palindrome(1, 4), "8");
        assert_eq!(Solution::largest_palindrome(5, 6), "89898");
    }
}
