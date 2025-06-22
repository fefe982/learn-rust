// https://leetcode.com/problems/russian-doll-envelopes/
// 354. Russian Doll Envelopes
pub struct Solution;
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_unstable_by(|v1, v2| (v1[0], -v1[1]).cmp(&(v2[0], -v2[1])));
        let mut r = vec![];
        for v in envelopes {
            if let Err(idx) = r.binary_search(&v[1]) {
                if idx == r.len() {
                    r.push(v[1]);
                } else {
                    r[idx] = v[1];
                }
            }
        }
        r.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_envelopes() {
        assert_eq!(Solution::max_envelopes(vec_vec![[5, 4], [6, 4], [6, 7], [2, 3]]), 3);
        assert_eq!(Solution::max_envelopes(vec_vec![[1, 1], [1, 1], [1, 1]]), 1);
    }
}
