// https://leetcode.com/problems/next-greater-element-iii/
// 556. Next Greater Element III
pub struct Solution;
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut s = n.to_string().chars().collect::<Vec<char>>();
        let l = s.len();
        for i in (1..l).rev() {
            if s[i] > s[i - 1] {
                let mut j = i;
                for k in i + 1..l {
                    if s[k] > s[i - 1] && s[k] <= s[j] {
                        j = k;
                    }
                }
                s.swap(i - 1, j);
                for k in 0..(l - i) / 2 {
                    s.swap(i + k, l - 1 - k);
                }
                return s.into_iter().collect::<String>().parse::<i32>().unwrap_or(-1);
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_greater_element() {
        assert_eq!(Solution::next_greater_element(12222333), 12223233);
        assert_eq!(Solution::next_greater_element(230241), 230412);
        assert_eq!(Solution::next_greater_element(12), 21);
        assert_eq!(Solution::next_greater_element(21), -1);
    }
}
