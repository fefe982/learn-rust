// https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/
// 1374. Generate a String With Characters That Have Odd Counts
pub struct Solution;
impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n % 2 == 0 {
            "a".repeat((n - 1) as usize) + "b"
        } else {
            "a".repeat(n as usize)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: i32) {
        let str = Solution::generate_the_string(n);
        let s = str.as_bytes();
        assert!(s.len() == n as usize);
        let mut c = vec![0; 26];
        for &b in s {
            assert!(b >= b'a' && b <= b'z');
            c[(b - b'a') as usize] += 1;
        }
        for &i in c.iter() {
            assert!(c[i] == 0 || c[i] % 2 == 1);
        }
    }
    #[test]
    fn generate_the_string() {
        check(4);
        check(2);
        check(7);
    }
}
