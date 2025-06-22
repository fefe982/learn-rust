// https://leetcode.com/problems/lexicographical-numbers/
// 386. Lexicographical Numbers
pub struct Solution;
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut num = 1;
        let mut res = Vec::with_capacity(n as usize);
        while num > 0 {
            res.push(num);
            if num * 10 <= n {
                num *= 10;
            } else {
                while num > 0 {
                    if num % 10 < 9 && num + 1 <= n {
                        num += 1;
                        break;
                    }
                    num /= 10;
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexical_order() {
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(Solution::lexical_order(2), vec![1, 2]);
    }
}
