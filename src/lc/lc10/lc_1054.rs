// https://leetcode.com/problems/distant-barcodes/
// 1054. Distant Barcodes
pub struct Solution;
impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut barcodes = barcodes;
        let mut cnt = vec![0; 10001];
        let mut max_cnt = 0;
        let mut max_cnt_i = 0;
        let l = barcodes.len();
        for &b in &barcodes {
            cnt[b as usize] += 1;
            if cnt[b as usize] > max_cnt {
                max_cnt = cnt[b as usize];
                max_cnt_i = b;
            }
        }
        if max_cnt == l / 2 + 1 {
            cnt[max_cnt_i as usize] = 0;
            let mut low = 1;
            for idx in 0..barcodes.len() / 2 {
                while cnt[low] == 0 {
                    low += 1;
                }
                barcodes[idx * 2] = max_cnt_i;
                barcodes[idx * 2 + 1] = low as i32;
                cnt[low] -= 1;
            }
            barcodes[l - 1] = max_cnt_i;
        } else {
            let mut low = 1;
            let mut high = 10000;
            for idx in 0..l / 2 {
                while cnt[low] == 0 {
                    low += 1;
                }
                barcodes[idx * 2 + 1] = low as i32;
                cnt[low] -= 1;
            }
            for idx in (0..=(l - 1) / 2).rev() {
                while cnt[high] == 0 {
                    high -= 1;
                }
                barcodes[idx * 2] = high as i32;
                cnt[high] -= 1;
            }
        }
        barcodes
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check_res(r: Vec<i32>) -> bool {
        for idx in 1..r.len() {
            if r[idx - 1] == r[idx] {
                return false;
            }
        }
        true
    }
    #[test]
    fn rearrange_barcodes() {
        assert!(check_res(Solution::rearrange_barcodes(vec![1, 1, 2])));
        assert!(check_res(Solution::rearrange_barcodes(vec![2, 2, 1, 3])));
        assert!(check_res(Solution::rearrange_barcodes(vec![1, 1, 1, 2, 2, 2])));
        assert!(check_res(Solution::rearrange_barcodes(vec![1, 1, 1, 1, 2, 2, 3, 3])));
    }
}
