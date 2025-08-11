// https://leetcode.com/problems/string-without-aaa-or-bbb/
// 984. String Without AAA or BBB
pub struct Solution;
impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let mut a = a;
        let mut b = b;
        let mut ans = String::new();
        while a > 0 || b > 0 {
            if a > b {
                if b == 0 {
                    ans.push_str("a".repeat(a as usize).as_str());
                    break;
                }
                ans.push_str("aab");
                a -= 2;
                b -= 1;
            } else if b > a {
                if a == 0 {
                    ans.push_str("b".repeat(b as usize).as_str());
                    break;
                }
                ans.push_str("bba");
                b -= 2;
                a -= 1;
            } else {
                ans.push_str("ab");
                a -= 1;
                b -= 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(a: i32, b: i32) {
        let ans = Solution::str_without3a3b(a, b);
        let a = a as usize;
        let b = b as usize;
        let mut cnt = [0, 0];
        assert!(ans.len() == a + b);
        let mut l = ' ';
        let mut cl = 0;
        for c in ans.chars() {
            if c == l {
                cl += 1;
                assert!(cl <= 3);
            }
            if c == 'a' {
                cnt[0] += 1;
            } else if c == 'b' {
                cnt[1] += 1;
            } else {
                assert!(false);
            }
            l = c;
        }
        assert_eq!(cnt[0], a);
        assert_eq!(cnt[1], b);
    }
    #[test]
    fn str_without3a3b() {
        check(1, 3);
        check(1, 2);
        check(4, 1);
    }
}
