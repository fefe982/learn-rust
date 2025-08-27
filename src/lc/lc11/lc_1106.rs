// https://leetcode.com/problems/parsing-a-boolean-expression/
// 1106. Parsing A Boolean Expression
pub struct Solution;
impl Solution {
    fn to_end(e: &[u8]) -> &[u8] {
        let mut cnt = 0;
        for i in 0..e.len() {
            if e[i] == b'(' {
                cnt += 1;
            } else if e[i] == b')' {
                if cnt == 0 {
                    return &e[i + 1..];
                }
                cnt -= 1;
            }
        }
        unreachable!()
    }
    fn parse_expr(e: &[u8]) -> (bool, &[u8]) {
        match e[0] {
            b't' => (true, &e[1..]),
            b'f' => (false, &e[1..]),
            b'!' => {
                assert!(e[1] == b'(');
                let (v, e) = Solution::parse_expr(&e[2..]);
                assert!(e[0] == b')');
                (!v, &e[1..])
            }
            b'&' => {
                assert!(e[1] == b'(');
                let mut v = true;
                let mut e = &e[2..];
                loop {
                    let (vi, ne) = Solution::parse_expr(e);
                    v &= vi;
                    if ne[0] == b')' {
                        break (v, &ne[1..]);
                    }
                    if v == false {
                        break (false, Self::to_end(ne));
                    }
                    assert!(ne[0] == b',');
                    e = &ne[1..];
                }
            }
            b'|' => {
                assert!(e[1] == b'(');
                let mut v = false;
                let mut e = &e[2..];
                loop {
                    let (vi, ne) = Solution::parse_expr(e);
                    v |= vi;
                    if ne[0] == b')' {
                        break (v, &ne[1..]);
                    }
                    if v == true {
                        break (true, Self::to_end(ne));
                    }
                    assert!(ne[0] == b',');
                    e = &ne[1..];
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn parse_bool_expr(expression: String) -> bool {
        let e = expression.as_bytes();
        Self::parse_expr(e).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_bool_expr() {
        assert_eq!(Solution::parse_bool_expr(String::from("!(&(!(t),&(f),|(f)))")), true);
        assert_eq!(Solution::parse_bool_expr(String::from("&(|(f))")), false);
        assert_eq!(Solution::parse_bool_expr(String::from("|(f,f,f,t)")), true);
        assert_eq!(Solution::parse_bool_expr(String::from("!(&(f,t))")), true);
    }
}
