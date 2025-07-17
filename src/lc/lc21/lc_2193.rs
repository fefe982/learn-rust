// https://leetcode.com/problems/minimum-number-of-moves-to-make-palindrome/
// 2193. Minimum Number of Moves to Make Palindrome
pub struct Solution;
impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut ans = 0;
        let mut l = 0;
        let mut r = s.len() - 1;
        let mut single = usize::MAX;
        while l < r {
            if s[l] != s[r] {
                let mut dl = usize::MAX;
                let mut dr = usize::MAX;
                for i in l + 1..=r - 1 {
                    if s[r] == s[i] {
                        dl = i - l;
                        break;
                    }
                    if s[l] == s[i] {
                        dr = r - i;
                    }
                }
                if dl == usize::MAX {
                    single = r;
                    r -= 1;
                    continue;
                }
                for i in (l + dl + 1..=r - 1).rev() {
                    if s[l] == s[i] {
                        dr = r - i;
                        break;
                    }
                }
                if dr == usize::MAX {
                    single = l;
                    l += 1;
                    continue;
                }
                if dl < dr {
                    ans += dl;
                    for i in (l + 1..=l + dl).rev() {
                        s[i] = s[i - 1];
                    }
                } else {
                    ans += dr;
                    for i in r - dr..=r - 1 {
                        s[i] = s[i + 1];
                    }
                }
            }
            l += 1;
            r -= 1;
        }
        if single != usize::MAX {
            let mid = s.len() / 2;
            if single > mid {
                ans += single - mid;
            } else {
                ans += mid - single;
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_moves_to_make_palindrome() {
        assert_eq!(
            Solution::min_moves_to_make_palindrome(String::from("skwhhaaunskegmdtutlgtteunmuuludii")),
            163
        );
        assert_eq!(Solution::min_moves_to_make_palindrome(String::from("aabb")), 2);
        assert_eq!(Solution::min_moves_to_make_palindrome(String::from("letelt")), 2);
    }
}
