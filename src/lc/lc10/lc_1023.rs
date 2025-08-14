// https://leetcode.com/problems/camelcase-matching/
// 1023. Camelcase Matching
pub struct Solution;
impl Solution {
    fn match_one(q: &[u8], p: &[u8]) -> bool {
        let mut idx_q = 0;
        let mut idx_p = 0;
        loop {
            let c = q[idx_q];
            if c >= b'A' && c <= b'Z' {
                if p[idx_p] != c {
                    return false;
                }
                idx_p += 1;
            } else {
                if p[idx_p] == c {
                    idx_p += 1;
                }
            }
            idx_q += 1;
            if idx_p >= p.len() || idx_q >= q.len() {
                break;
            }
        }
        if idx_p >= p.len() {
            while idx_q < q.len() {
                let c = q[idx_q];
                if c >= b'A' && c <= b'Z' {
                    return false;
                }
                idx_q += 1;
            }
        } else if idx_p < p.len() {
            return false;
        }
        true
    }
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let p = pattern.as_bytes();
        queries
            .iter()
            .map(|q| Self::match_one(q.as_bytes(), p))
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn camel_match() {
        assert_eq!(
            Solution::camel_match(
                [
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ]
                .into_iter()
                .map(|x| String::from(x))
                .collect(),
                String::from("FB")
            ),
            vec![true, false, true, true, false]
        );
        assert_eq!(
            Solution::camel_match(
                [
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ]
                .into_iter()
                .map(|x| String::from(x))
                .collect(),
                String::from("FoBa")
            ),
            vec![true, false, true, false, false]
        );
        assert_eq!(
            Solution::camel_match(
                [
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ]
                .into_iter()
                .map(|x| String::from(x))
                .collect(),
                String::from("FoBaT")
            ),
            vec![false, true, false, false, false]
        );
    }
}
