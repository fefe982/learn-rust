// https://leetcode.com/problems/probability-of-a-two-boxes-having-the-same-number-of-distinct-balls/
// 1467. Probability of a Two Boxes Having The Same Number of Distinct Balls
pub struct Solution;

impl Solution {
    fn frac(i: i32, frac_v: &mut Vec<f64>) -> f64 {
        if frac_v.len() > i as usize {
            return frac_v[i as usize];
        }
        let ans = i as f64 * Self::frac(i - 1, frac_v);
        frac_v.push(ans);
        ans
    }
    fn permute(v: &Vec<i32>, frac_v: &mut Vec<f64>) -> f64 {
        let mut denorm = 1.0;
        let mut sum = 0;
        for &n in v {
            denorm *= Self::frac(n, frac_v);
            sum += n;
        }
        Self::frac(sum, frac_v) / denorm
    }
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let mut frac_v = vec![1.0, 1.0];
        let total = Self::permute(&balls, &mut frac_v);
        let mut diff = balls.iter().sum::<i32>() / 2;
        let mut left = vec![0; balls.len()];
        let mut right = balls.clone();
        let mut good = 0.0;
        loop {
            if diff == 0 {
                let mut idx = 0;
                for i in 0..balls.len() {
                    if left[i] != 0 {
                        diff += left[i];
                        right[i] += left[i];
                        left[i] = 0;
                        idx = i + 1;
                        break;
                    }
                }
                while idx < balls.len() {
                    if right[idx] == 0 {
                        right[idx] = left[idx];
                        left[idx] = 0;
                        diff += right[idx];
                        idx += 1;
                    } else {
                        diff -= 1;
                        left[idx] += 1;
                        right[idx] -= 1;
                        break;
                    }
                }
                if idx == balls.len() {
                    return good / total;
                }
            }
            for i in 0..balls.len() {
                if right[i] > 0 {
                    let mv = right[i].min(diff);
                    diff -= mv;
                    right[i] -= mv;
                    left[i] += mv;
                    if diff == 0 {
                        break;
                    }
                }
            }
            if diff == 0 {
                if left.iter().filter(|&&x| x > 0).count() == right.iter().filter(|&&x| x > 0).count() {
                    good += Self::permute(&left, &mut frac_v) * Self::permute(&right, &mut frac_v);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_get_probability() {
        assert_approx_eq!(Solution::get_probability(vec![1, 1]), 1.0, 1e-5);
        assert_approx_eq!(Solution::get_probability(vec![2, 1, 1]), 0.6666666666, 1e-5);
        assert_approx_eq!(Solution::get_probability(vec![1, 2, 1, 2]), 0.6, 1e-5);
    }
}
