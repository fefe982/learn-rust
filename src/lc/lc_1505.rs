// https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits/
// 1505. Minimum Possible Integer After at Most K Adjacent Swaps On Digits
pub struct Solution;
impl Solution {
    fn add(arr: &mut Vec<i32>, mut pos: usize, add: i32) {
        while pos < arr.len() {
            arr[pos] += add;
            pos += pos & (!pos + 1);
        }
    }
    fn cnt(arr: &Vec<i32>, mut pos: usize) -> i32 {
        let mut ans = 0;
        while pos > 0 {
            ans += arr[pos];
            pos -= pos & (!pos + 1);
        }
        ans
    }
    pub fn min_integer(num: String, k: i32) -> String {
        let num = num.as_bytes();
        let mut g = vec![vec![]; 10];
        let mut arr = vec![0; num.len() + 1];
        for (i, &c) in num.iter().enumerate() {
            g[(c - b'0') as usize].push(i);
            Self::add(&mut arr, i + 1, 1);
        }
        let mut used = vec![false; num.len()];
        let mut ans = vec![];
        let mut k = k;
        let mut r = num.len();
        for c in 0..10 {
            for &i in &g[c] {
                if i > r {
                    break;
                }
                let cnt = Self::cnt(&arr, i + 1);
                if cnt <= k + 1 {
                    used[i] = true;
                    Self::add(&mut arr, i + 1, -1);
                    k -= cnt as i32 - 1;
                    ans.push((b'0' + c as u8) as char);
                } else {
                    r = i;
                    break;
                }
            }
        }
        for (i, u) in used.into_iter().enumerate() {
            if !u {
                ans.push(num[i] as char);
                let mut ui = ans.len() - 1;
                while k > 0 && ui > 0 && ans[ui - 1] > ans[ui] {
                    ans.swap(ui - 1, ui);
                    k -= 1;
                    ui -= 1;
                }
            }
        }
        ans.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_integer() {
        assert_eq!(Solution::min_integer(String::from("4321"), 6), "1234");
        assert_eq!(Solution::min_integer(String::from("294984148179"), 11), "124498948179");
        assert_eq!(Solution::min_integer(String::from("4321"), 4), "1342");
        assert_eq!(Solution::min_integer(String::from("100"), 1), "010");
        assert_eq!(Solution::min_integer(String::from("36789"), 3), "36789");
    }
}
