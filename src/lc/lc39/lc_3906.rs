// https://leetcode.com/problems/count-good-integers-on-a-grid-path/
// 3906. Count Good Integers on a Grid Path
pub struct Solution;
impl Solution {
    fn build_path_mask(directions: &str) -> [bool; 16] {
        let mut mask = [false; 16];
        let (mut row, mut col) = (0usize, 0usize);
        mask[0] = true;
        for ch in directions.bytes() {
            match ch {
                b'D' => row += 1,
                b'R' => col += 1,
                _ => unreachable!(),
            }
            mask[row * 4 + col] = true;
        }
        mask
    }

    fn digits_16(mut value: i64) -> [u8; 16] {
        let mut digits = [0u8; 16];
        for idx in (0..16).rev() {
            digits[idx] = (value % 10) as u8;
            value /= 10;
        }
        digits
    }

    fn count_up_to(limit: i64, path_mask: &[bool; 16]) -> i64 {
        if limit < 0 {
            return 0;
        }

        let digits = Self::digits_16(limit);
        let mut dp = [[0i64; 11]; 2];
        dp[1][10] = 1;

        for pos in 0..16 {
            let mut next = [[0i64; 11]; 2];
            for tight in 0..=1 {
                for last_digit in 0..=10 {
                    let ways = dp[tight][last_digit];
                    if ways == 0 {
                        continue;
                    }
                    let upper = if tight == 1 { digits[pos] } else { 9 };
                    for digit in 0..=upper {
                        if path_mask[pos] && last_digit != 10 && digit < last_digit as u8 {
                            continue;
                        }
                        let next_tight = (tight == 1 && digit == upper) as usize;
                        let next_last = if path_mask[pos] { digit as usize } else { last_digit };
                        next[next_tight][next_last] += ways;
                    }
                }
            }
            dp = next;
        }

        dp.into_iter().flatten().sum()
    }

    pub fn count_good_integers_on_path(l: i64, r: i64, directions: String) -> i64 {
        let path_mask = Self::build_path_mask(&directions);
        Self::count_up_to(r, &path_mask) - Self::count_up_to(l - 1, &path_mask)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_good_integers_on_path() {
        assert_eq!(Solution::count_good_integers_on_path(8, 10, "DDDRRR".to_string()), 2);
        assert_eq!(
            Solution::count_good_integers_on_path(123456789, 123456790, "DDRRDR".to_string()),
            1
        );
        assert_eq!(
            Solution::count_good_integers_on_path(1288561398769758, 1288561398769758, "RRRDDD".to_string()),
            0
        );
    }
}
