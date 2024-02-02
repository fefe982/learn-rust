// https://leetcode.com/problems/sequential-digits/
// 1291. Sequential Digits
pub struct Solution;
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut s = 0;
        let mut diff = 0;
        let mut ans = vec![];
        for i in 1..10 {
            s = s * 10 + i;
            diff = diff * 10 + 1;
            if s > high {
                break;
            }
            if s >= low / 10 {
                for j in 0..9 - i + 1 {
                    let t = s + diff * j;
                    if t < low {
                        continue;
                    }
                    if t <= high {
                        ans.push(t);
                    } else {
                        break;
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
    fn test_sequential_digits() {
        assert_eq!(Solution::sequential_digits(58, 155), vec![67, 78, 89, 123]);
        assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
        assert_eq!(
            Solution::sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }
}
