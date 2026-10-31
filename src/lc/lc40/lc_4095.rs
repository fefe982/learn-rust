// https://leetcode.com/problems/count-frequency-staircase-subarrays-ii/
// 4095. Count Frequency Staircase Subarrays II
pub struct Solution;
impl Solution {
    pub fn count_staircase(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut pos = std::collections::HashMap::new();
        let mut firsts = vec![];
        let mut ans = 0;
        for i in (0..n).rev() {
            let x = nums[i];
            (&mut pos).entry(x).or_insert(vec![]).push(i);
            let xi = firsts.iter().position(|&e| e == x).unwrap_or(firsts.len());
            if xi == firsts.len() {
                firsts.push(0);
            }
            for j in (1..=xi).rev() {
                firsts[j] = firsts[j - 1];
            }
            firsts[0] = x;
            let m = firsts.len();
            if m * (m + 1) / 2 > n {
                firsts.pop();
            }
            let mut maxx = 0;
            let mut minn = n;
            let mut r = i;
            for (&x, k) in firsts.iter().zip(1..) {
                r += k;
                let ps = pos.get(&x).unwrap();
                let psl = ps.len();
                if k > psl {
                    break;
                }
                maxx = maxx.max(ps[psl - k]);
                if k < psl {
                    minn = minn.min(ps[psl - k - 1]);
                }
                if maxx >= minn || r > minn {
                    break;
                }
                if maxx < r {
                    ans += 1;
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
    fn count_staircase() {
        assert_eq!(Solution::count_staircase(vec![5, 8, 8, 2, 2, 2]), 9);
        assert_eq!(Solution::count_staircase(vec![6, 6, 9, 9]), 5);
        assert_eq!(Solution::count_staircase(vec![2, 3, 4, 4, 3, 4]), 8);
    }
}
