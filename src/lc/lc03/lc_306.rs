// https://leetcode.com/problems/additive-number/
// 306. Additive Number
pub struct Solution;
impl Solution {
    fn check_add(num: &Vec<i32>, split1: (usize, usize), split2: (usize, usize), split3: (usize, usize)) -> bool {
        let mut i = 0;
        let mut c = 0;
        let min = split1.1.min(split2.1);
        if split3.0 + split3.1 > num.len() {
            return false;
        }
        while i < min {
            let d = num[split1.0 + split1.1 - i - 1] + num[split2.0 + split2.1 - i - 1] + c;
            c = d / 10;
            if d % 10 != num[split3.0 + split3.1 - i - 1] {
                return false;
            }
            i += 1;
        }
        while i < split1.1 {
            let d = num[split1.0 + split1.1 - i - 1] + c;
            c = d / 10;
            if d % 10 != num[split3.0 + split3.1 - i - 1] {
                return false;
            }
            i += 1;
        }
        while i < split2.1 {
            let d = num[split2.0 + split2.1 - i - 1] + c;
            c = d / 10;
            if d % 10 != num[split3.0 + split3.1 - i - 1] {
                return false;
            }
            i += 1;
        }
        (c == 0 && i == split3.1) || (c == 1 && num[split3.0] == 1 && i + 1 == split3.1)
    }
    fn check(num: &Vec<i32>, split1: (usize, usize), split2: (usize, usize)) -> bool {
        if split2.0 + split2.1 == num.len() {
            return true;
        }
        let n = split1.1.max(split2.1);
        if split2.0 + split2.1 + n > num.len() {
            return false;
        }
        if Self::check_add(num, split1, split2, (split2.0 + split2.1, n))
            && Self::check(num, split2, (split2.0 + split2.1, n))
        {
            return true;
        }
        split2.0 + split2.1 + n + 1 <= num.len()
            && num[split2.0 + split2.1] == 1
            && Self::check_add(num, split1, split2, (split2.0 + split2.1, n + 1))
            && Self::check(num, split2, (split2.0 + split2.1, n + 1))
    }
    pub fn is_additive_number(num: String) -> bool {
        let num = num.as_bytes().iter().map(|&c| (c - b'0') as i32).collect::<Vec<_>>();
        let n = num.len();
        if n < 3 {
            return false;
        }
        for i in 1..=n / 2 {
            if i > 1 && num[0] == 0 {
                break;
            }
            for j in 1..=n / 2 {
                if j > 1 && num[i] == 0 {
                    break;
                }
                let maxlen = i.max(j);
                if i + j + maxlen > n {
                    break;
                }
                if Self::check(&num, (0, i), (i, j)) {
                    return true;
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_additive_number() {
        assert_eq!(Solution::is_additive_number("121474836472147483648".to_string()), true);
        assert_eq!(Solution::is_additive_number("101".to_string()), true);
        assert_eq!(Solution::is_additive_number("011112".to_string()), false);
        assert_eq!(Solution::is_additive_number("112358".to_string()), true);
        assert_eq!(Solution::is_additive_number("199100199".to_string()), true);
    }
}
