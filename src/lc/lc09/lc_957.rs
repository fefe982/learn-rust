// https://leetcode.com/problems/prison-cells-after-n-days/
// 957. Prison Cells After N Days
pub struct Solution;
impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut k = 0;
        for i in 0..cells.len() {
            k = k << 1;
            if cells[i] == 1 {
                k += 1;
            }
        }
        let mut idx = vec![0; 65];
        idx[0] = k;
        let mut seen = std::collections::HashMap::new();
        seen.insert(k, 0);
        let n = n as usize;
        for i in 1..=n {
            let mut nk = 0;
            let mut needle = 0b10100000;
            while needle >= 5 {
                nk <<= 1;
                let check = k & needle;
                if check == needle || check == 0 {
                    nk += 2;
                }
                needle >>= 1;
            }
            if let Some(&j) = seen.get(&nk) {
                k = idx[j + (n - j) % (i - j)];
                break;
            }
            k = nk;
            idx[i] = k;
            seen.insert(k, i);
        }
        let mut ans = vec![0; cells.len()];
        for i in 0..cells.len() {
            if (k & 1) == 1 {
                ans[cells.len() - 1 - i] = 1;
            }
            k >>= 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prison_after_n_days() {
        assert_eq!(
            Solution::prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
            [0, 0, 1, 1, 0, 0, 0, 0]
        );
        assert_eq!(
            Solution::prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
            [0, 0, 1, 1, 1, 1, 1, 0]
        );
    }
}
