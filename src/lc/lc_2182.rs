// https://leetcode.com/problems/construct-string-with-repeat-limit/
// 2182. Construct String With Repeat Limit
pub struct Solution;
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut ans = String::new();
        let mut cnt = vec![0; 26];
        for &c in s.as_bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        for i in (0..26).rev() {
            while cnt[i] > 0 {
                let c = (b'a' + i as u8) as char;
                let r = repeat_limit.min(cnt[i]);
                for _ in 0..r {
                    ans.push(c);
                }
                cnt[i] -= r;
                if cnt[i] > 0 {
                    let mut found = false;
                    for j in (0..i).rev() {
                        if cnt[j] > 0 {
                            ans.push((b'a' + j as u8) as char);
                            found = true;
                            cnt[j] -= 1;
                            break;
                        }
                    }
                    if !found {
                        return ans;
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
    #[test]
    fn test_repeat_limited_string() {
        assert_eq!(Solution::repeat_limited_string("cczazcc".to_string(), 3), "zzcccac");
        assert_eq!(Solution::repeat_limited_string("aababab".to_string(), 2), "bbabaa");
    }
}
