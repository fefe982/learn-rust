// https://leetcode.com/problems/pancake-sorting/
// 969. Pancake Sorting
pub struct Solution;
impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut idx = vec![0; n];
        for i in 0..n {
            idx[arr[i] as usize - 1] = i;
        }
        let mut ans = vec![];
        for i in (0..n).rev() {
            let pos = idx[i];
            if pos == i {
                continue;
            }
            if pos != 0 {
                ans.push(pos as i32 + 1);
            }
            ans.push(i as i32 + 1);
            for j in 0..=i {
                if idx[j] <= pos {
                    idx[j] = i + idx[j] - pos;
                } else {
                    idx[j] = i - idx[j];
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn get_pos(mut pos: usize, arr: &Vec<i32>) -> usize {
        for &k in arr {
            let k = k as usize;
            if pos >= k {
                continue;
            }
            pos = k - 1 - pos;
        }
        pos
    }
    fn check(arr: Vec<i32>) {
        let ans = Solution::pancake_sort(arr.clone());
        println!("{arr:?} -> {ans:?}");
        assert!(ans.len() <= arr.len() * 10);
        for i in 0..arr.len() {
            assert_eq!(get_pos(i, &ans), arr[i] as usize - 1);
        }
    }
    #[test]
    fn pancake_sort() {
        check(vec![3, 2, 4, 1]);
        check(vec![1, 2, 3]);
    }
}
