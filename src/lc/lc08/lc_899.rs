// https://leetcode.com/problems/orderly-queue/description/
// 899. Orderly Queue
pub struct Solution;
impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut v = s.chars().collect::<Vec<_>>();
        if k == 1 {
            let n = v.len();
            let mut vc = v.clone();
            v.append(&mut vc);
            let mut maxi = 0;
            for i in 1..n {
                if v[i..i + n] < v[maxi..maxi + n] {
                    maxi = i;
                }
            }
            return v[maxi..maxi + n].iter().collect();
        }
        v.sort_unstable();
        v.into_iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_orderly_queue() {
        assert_eq!(Solution::orderly_queue("xisxr".to_string(), 1), "isxrx");
        assert_eq!(Solution::orderly_queue("cba".to_string(), 1), "acb");
        assert_eq!(Solution::orderly_queue("baaca".to_string(), 3), "aaabc");
    }
}
