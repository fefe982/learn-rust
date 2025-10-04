// https://leetcode.com/problems/four-divisors/
// 1390. Four Divisors
pub struct Solution;
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut prime = vec![true; 316];
        prime[0] = false;
        prime[1] = false;
        for i in 2..=17 {
            if prime[i as usize] {
                for j in (i * i..316).step_by(i as usize) {
                    prime[j as usize] = false;
                }
            }
        }
        let primev = prime
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| if b { Some(i as i32) } else { None })
            .collect::<Vec<_>>();
        let mut sum = 0;
        for num in nums {
            let mut n = num;
            let mut invalid = false;
            for &p in &primev {
                if n % p == 0 {
                    if num != n {
                        if n == p {
                            sum += 1 + n + num / n + num;
                        }
                        invalid = true;
                        break;
                    } else {
                        if n == p {
                            invalid = true;
                            break;
                        }
                        n = n / p;
                        if n % p == 0 {
                            if n / p == p {
                                sum += 1 + p + n + num;
                            }
                            invalid = true;
                            break;
                        }
                    }
                }
            }
            if num != n && !invalid {
                sum += 1 + n + num / n + num;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn four_divisors() {
        assert_eq!(Solution::sum_four_divisors(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 45);
        assert_eq!(Solution::sum_four_divisors(vec![21, 4, 7]), 32);
        assert_eq!(Solution::sum_four_divisors(vec![21, 21]), 64);
        assert_eq!(Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]), 0);
    }
}
