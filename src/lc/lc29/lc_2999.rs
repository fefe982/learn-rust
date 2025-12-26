// https://leetcode.com/problems/count-the-number-of-powerful-integers/
// 2999. Count the Number of Powerful Integers
pub struct Solution;
impl Solution {
    fn count(bound: &Vec<i32>, limit: i32, s: &Vec<i32>, cnt: &mut Vec<i64>) -> i64 {
        if bound.len() < s.len() {
            return 0;
        }
        Self::count_(bound, bound.len() - 1, true, limit, s, cnt)
    }
    fn count_(bound: &Vec<i32>, idigit: usize, strict: bool, limit: i32, s: &Vec<i32>, cnt: &mut Vec<i64>) -> i64 {
        if strict == false {
            if idigit + 1 == s.len() {
                return 1;
            }
            if cnt[idigit] != -1 {
                return cnt[idigit];
            }
            let res = Self::count_(bound, idigit - 1, false, limit, s, cnt) * (limit + 1) as i64;
            cnt[idigit] = res;
            return res;
        }
        if idigit + 1 > s.len() {
            Self::count_(bound, idigit - 1, false, limit, s, cnt) * (bound[idigit]).min(limit + 1) as i64
                + if bound[idigit] <= limit {
                    Self::count_(bound, idigit - 1, true, limit, s, cnt)
                } else {
                    0
                }
        } else {
            match bound[idigit].cmp(&s[idigit]) {
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Equal => {
                    if idigit == 0 {
                        1
                    } else {
                        Self::count_(bound, idigit - 1, true, limit, s, cnt)
                    }
                }
                std::cmp::Ordering::Less => 0,
            }
        }
    }
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let mut cnt = vec![-1; 64];
        let cvt = |x: i64| -> Vec<i32> {
            let mut start = x;
            let mut res = vec![];
            while start > 0 {
                res.push((start % 10) as i32);
                start /= 10;
            }
            res
        };
        let start = cvt(start - 1);
        let finish = cvt(finish);
        let s = s
            .as_bytes()
            .iter()
            .map(|x| (x - b'0') as i32)
            .rev()
            .collect::<Vec<i32>>();
        Self::count(&finish, limit, &s, &mut cnt) - Self::count(&start, limit, &s, &mut cnt)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_powerful_int() {
        assert_eq!(Solution::number_of_powerful_int(10, 1844, 5, "12".to_string()), 12);
        assert_eq!(Solution::number_of_powerful_int(1, 971, 9, "17".to_string()), 10);
        assert_eq!(Solution::number_of_powerful_int(1, 6000, 4, "124".to_string()), 5);
        assert_eq!(Solution::number_of_powerful_int(15, 215, 6, "10".to_string()), 2);
        assert_eq!(Solution::number_of_powerful_int(1000, 2000, 4, "3000".to_string()), 0);
    }
}
