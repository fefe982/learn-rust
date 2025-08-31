// https://leetcode.com/problems/alphabet-board-path/
// 1138. Alphabet Board Path
pub struct Solution;
impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut x = 0;
        let mut y = 0;
        let mut res = String::new();
        for c in target.chars() {
            let idx = c as u8 - b'a';
            let nx = idx % 5;
            let ny = idx / 5;
            if y > ny {
                for _ in 0..y - ny {
                    res.push('U');
                }
            }
            if x > nx {
                for _ in 0..x - nx {
                    res.push('L');
                }
            }
            if y < ny {
                for _ in 0..ny - y {
                    res.push('D');
                }
            }
            if x < nx {
                for _ in 0..nx - x {
                    res.push('R');
                }
            }
            res.push('!');
            x = nx;
            y = ny;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(target: String, expect: String) {
        let res = Solution::alphabet_board_path(target.clone());
        assert_eq!(res.len(), expect.len());
        let mut x = 0;
        let mut y = 0;
        let mut i = 0;
        let target = target.as_bytes();
        for c in res.chars() {
            match c {
                'U' => {
                    assert!(y > 0);
                    y -= 1;
                }
                'D' => {
                    assert!(y < 4 || (y == 4 && x == 0));
                    y += 1;
                }
                'L' => {
                    assert!(x > 0);
                    x -= 1;
                }
                'R' => {
                    assert!(x < 4 && y < 5);
                    x += 1;
                }
                '!' => {
                    assert!(i < target.len());
                    let c2 = b'a' + (y * 5 + x) as u8;
                    assert_eq!(c2, target[i]);
                    i += 1;
                }
                _ => assert!(false),
            }
        }
        assert_eq!(i, target.len());
    }
    #[test]
    fn alphabet_board_path() {
        check("zdz".to_string(), "DDDDD!UUUUURRR!DDDDLLLD!".to_string());
        check("zb".to_string(), "DDDDD!UUUUUR!".to_string());
        check("leet".to_string(), "DDR!UURRR!!DDD!".to_string());
        check("code".to_string(), "RR!DDRR!UUL!R!".to_string());
    }
}
