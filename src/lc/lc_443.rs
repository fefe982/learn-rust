// https://leetcode.com/problems/string-compression/
// 443. String Compression
pub struct Solution;
impl Solution {
    fn do_compress(chars: &mut Vec<char>, idx: &mut usize, ch: char, cnt: i32) {
        chars[*idx] = ch;
        *idx += 1;
        if cnt == 1 {
            return;
        }
        let cnt_str = cnt.to_string();
        for c in cnt_str.as_bytes() {
            chars[*idx] = *c as char;
            *idx += 1;
        }
    }
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut cnt = 0;
        let mut last: char = 0 as char;
        let len = chars.len();
        let mut idx: usize = 0;
        for i in 0..len {
            let c = chars[i];
            if c == last {
                cnt += 1;
            } else {
                if last != 0 as char {
                    Self::do_compress(chars, &mut idx, last, cnt);
                }
                last = c;
                cnt = 1;
            }
        }
        Self::do_compress(chars, &mut idx, last, cnt);
        idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compress() {
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']),
            6
        );
        assert_eq!(Solution::compress(&mut vec!['a']), 1);
        assert_eq!(
            Solution::compress(&mut vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
            ]),
            4
        );
    }
}
