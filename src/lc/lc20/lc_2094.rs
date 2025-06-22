// https://leetcode.com/problems/finding-3-digit-even-numbers/
// 2094. Finding 3-Digit Even Numbers
pub struct Solution;
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut cnt = [0; 10];
        for i in digits {
            cnt[i as usize] += 1;
        }
        for i in 1..10 {
            if cnt[i] > 0 {
                cnt[i] -= 1;
                for j in 0..10 {
                    if cnt[j] > 0 {
                        cnt[j] -= 1;
                        for k in 0..5 {
                            if cnt[k * 2] > 0 {
                                res.push((i * 100 + j * 10 + k * 2) as i32);
                            }
                        }
                        cnt[j] += 1;
                    }
                }
                cnt[i] += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_even_numbers() {
        assert_eq!(
            Solution::find_even_numbers(vec![2, 1, 3, 0]),
            [102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
        );
        assert_eq!(
            Solution::find_even_numbers(vec![2, 2, 8, 8, 2]),
            [222, 228, 282, 288, 822, 828, 882]
        );
        assert_eq!(Solution::find_even_numbers(vec![3, 7, 5]), []);
    }
}
