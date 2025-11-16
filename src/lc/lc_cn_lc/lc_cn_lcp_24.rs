// https://leetcode.cn/problems/5TxKeK/
// LCP 24. 数字游戏
pub struct Solution;
impl Solution {
    pub fn nums_game(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut sum = 0i64;
        let mut cnt = std::collections::HashMap::<i64, i64>::new();
        let mut low = std::collections::BTreeSet::<i64>::new();
        let mut cnt_low = 0;
        let mut high = low.clone();
        let mut cnt_high = 0;
        let mut center = nums[0] as i64;
        for n in nums.into_iter().enumerate().map(|(i, x)| x as i64 - i as i64) {
            *cnt.entry(n).or_default() += 1;
            if n < center {
                low.insert(n);
                cnt_low += 1;
            } else if n > center {
                high.insert(n);
                cnt_high += 1;
            }
            sum += (n - center).abs();
            while cnt_low > cnt_high + cnt.get(&center).unwrap() {
                let &l = low.range(..center).rev().next().unwrap();
                sum -= (cnt_low - cnt_high - cnt.get(&center).unwrap()) * (center - l);
                low.remove(&l);
                cnt_low -= cnt.get(&l).unwrap();
                high.insert(center);
                cnt_high += cnt.get(&center).unwrap();
                center = l;
            }
            while cnt_high > cnt_low + cnt.get(&center).unwrap() {
                let &h = high.range(center..).next().unwrap();
                sum -= (cnt_high - cnt_low - cnt.get(&center).unwrap()) * (h - center);
                high.remove(&h);
                cnt_high -= cnt.get(&h).unwrap();
                low.insert(center);
                cnt_low += cnt.get(&center).unwrap();
                center = h;
            }
            ans.push(sum)
        }
        ans.into_iter().map(|x| (x % 1_0000_0000_7) as i32).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nums_game() {
        assert_eq!(Solution::nums_game(vec![3, 4, 5, 1, 6, 7]), vec![0, 0, 0, 5, 6, 7]);
        assert_eq!(Solution::nums_game(vec![1, 2, 3, 4, 5]), vec![0, 0, 0, 0, 0]);
        assert_eq!(Solution::nums_game(vec![1, 1, 1, 2, 3, 4]), vec![0, 1, 2, 3, 3, 3]);
    }
}
