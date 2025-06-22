// https://leetcode.com/problems/self-dividing-numbers/
// 728. Self Dividing Numbers
pub struct Solution;
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = vec![];
        'i: for i in left..=right {
            if i < 10 {
                res.push(i);
            } else if i % 10 == 0 {
                continue;
            } else {
                let mut ii = i;
                while ii > 0 {
                    let d = ii % 10;
                    ii = ii / 10;
                    if d == 0 || i % d != 0 {
                        continue 'i;
                    }
                }
                res.push(i)
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn self_dividing_numbers() {
        assert_eq!(
            Solution::self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        assert_eq!(Solution::self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
    }
}
