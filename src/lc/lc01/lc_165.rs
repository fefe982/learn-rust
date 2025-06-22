// https://leetcode.com/problems/compare-version-numbers/
// 165. Compare Version Numbers
pub struct Solution;
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = version1.split('.').map(|s| s.parse::<i32>().unwrap());
        let mut v2 = version2.split('.').map(|s| s.parse::<i32>().unwrap());
        loop {
            let v1_oval = v1.next();
            let v2_oval = v2.next();
            match (v1_oval, v2_oval) {
                (None, None) => return 0,
                _ => {
                    let v1_val = v1_oval.unwrap_or(0);
                    let v2_val = v2_oval.unwrap_or(0);
                    if v1_val < v2_val {
                        return -1;
                    } else if v1_val > v2_val {
                        return 1;
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compare_version() {
        assert_eq!(Solution::compare_version("1.01".to_string(), "1.001".to_string()), 0);
        assert_eq!(Solution::compare_version("1.0".to_string(), "1.0.0".to_string()), 0);
        assert_eq!(Solution::compare_version("0.1".to_string(), "1.1".to_string()), -1);
    }
}
