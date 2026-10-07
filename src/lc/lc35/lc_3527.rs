// https://leetcode.com/problems/find-the-most-common-response/
// 3527. Find the Most Common Response
pub struct Solution;
impl Solution {
    pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
        let mut ans = String::new();
        let mut count = 0;
        let mut map = std::collections::HashMap::new();
        for response in responses {
            let mut set = std::collections::HashSet::new();
            for r in response {
                if set.insert(r.clone()) {
                    let c = map.entry(r.clone()).or_insert(0);
                    *c += 1;
                    if *c > count || (*c == count && r < ans) {
                        ans = r.clone();
                        count = *c;
                    }
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_common_response() {
        assert_eq!(
            Solution::find_common_response(vec_vec_str![
                ["good", "ok", "good", "ok"],
                ["ok", "bad", "good", "ok", "ok"],
                ["good"],
                ["bad"]
            ]),
            "good".to_string()
        );
        assert_eq!(
            Solution::find_common_response(vec_vec_str![
                ["good", "ok", "good"],
                ["ok", "bad"],
                ["bad", "notsure"],
                ["great", "good"]
            ]),
            "bad".to_string()
        );
    }
}
