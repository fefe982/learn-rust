// https://leetcode.com/problems/split-message-based-on-limit/
// 2468. Split Message Based on Limit
pub struct Solution;
impl Solution {
    pub fn split_message(message: String, limit: i32) -> Vec<String> {
        if limit < 6 {
            return vec![];
        }
        let mut digit = 1;
        let len = message.len();
        let mut segs;
        loop {
            segs = 0;
            let mut remain = limit - 4 - digit;
            let mut left = len as i32;
            let mut max_cnt = 9;
            let mut i = 1;
            loop {
                let cnt = (left + remain - 1) / remain;
                if cnt <= max_cnt {
                    segs += cnt;
                    break;
                } else {
                    left -= max_cnt * remain;
                    segs += max_cnt;
                    max_cnt *= 10;
                    i += 1;
                    remain -= 1;
                    if remain <= 0 {
                        return vec![];
                    }
                }
            }
            if i == digit {
                break;
            }
            if i > digit {
                digit += 1;
            }
            if i < digit {
                return vec![];
            }
        }
        let mut s = 0;
        let mut idx = 1;
        let mut ans = vec![];
        let mut cnt = 9;
        let mut seg_left = segs;
        for i in 1..=digit {
            let seg_len = limit - 3 - i - digit;
            for _ in 0..seg_left.min(cnt) {
                let e = (s + seg_len as usize).min(len);
                ans.push(format!("{}<{}/{}>", &message[s..e], idx, segs));
                s = e;
                idx += 1;
            }
            seg_left -= cnt;
            cnt *= 10;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn split_message() {
        assert_eq!(
            Solution::split_message("this is really a very awesome message".to_string(), 9),
            vec_str![
                "thi<1/14>",
                "s i<2/14>",
                "s r<3/14>",
                "eal<4/14>",
                "ly <5/14>",
                "a v<6/14>",
                "ery<7/14>",
                " aw<8/14>",
                "eso<9/14>",
                "me<10/14>",
                " m<11/14>",
                "es<12/14>",
                "sa<13/14>",
                "ge<14/14>"
            ]
        );
        assert_eq!(
            Solution::split_message("short message".to_string(), 15),
            vec_str!["short mess<1/2>", "age<2/2>"]
        );
    }
}
