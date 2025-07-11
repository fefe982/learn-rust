// https://leetcode.com/problems/number-of-valid-clock-times/
// 2437. Number of Valid Clock Times
pub struct Solution;
impl Solution {
    pub fn count_time(time: String) -> i32 {
        let time = time.as_bytes();
        let mut cnt = 1;
        match time[0] {
            b'0'..=b'1' => {
                if time[1] == b'?' {
                    cnt *= 10;
                }
            }
            b'2' => {
                if time[1] == b'?' {
                    cnt *= 4;
                }
            }
            b'?' => match time[1] {
                b'0'..=b'3' => cnt *= 3,
                b'4'..=b'9' => cnt *= 2,
                b'?' => cnt *= 24,
                _ => (),
            },
            _ => (),
        }
        match time[3] {
            b'0'..=b'5' => {
                if time[4] == b'?' {
                    cnt *= 10;
                }
            }
            b'?' => match time[4] {
                b'0'..=b'9' => cnt *= 6,
                b'?' => cnt *= 60,
                _ => (),
            },
            _ => (),
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_time() {
        assert_eq!(Solution::count_time(String::from("?5:00")), 2);
        assert_eq!(Solution::count_time(String::from("0?:0?")), 100);
        assert_eq!(Solution::count_time(String::from("??:??")), 1440);
    }
}
