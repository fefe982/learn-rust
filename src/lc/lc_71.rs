// https://leetcode.com/problems/simplify-path/
// 71. Simplify Path
pub struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path = path.as_bytes();
        let mut stk: Vec<&[u8]> = Vec::new();
        let mut last = 0;
        for (idx, &c) in path.iter().chain(b"/".iter()).enumerate() {
            if c == b'/' {
                if last != 0 && last != idx {
                    let n = &path[last..idx];
                    if n == b".." {
                        stk.pop();
                    } else if n != b"." {
                        stk.push(n);
                    }
                }
                last = idx + 1;
            }
        }
        if stk.is_empty() {
            return String::from("/");
        }
        let mut path = Vec::new();
        for dir in stk {
            path.push(b'/');
            path.extend(dir);
        }
        String::from_utf8(path).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simplify_path() {
        assert_eq!(
            Solution::simplify_path(String::from("/home/")),
            String::from("/home")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home//foo/")),
            String::from("/home/foo")
        );
    }
}
