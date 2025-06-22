// https://leetcode.com/problems/create-maximum-number/
// 321. Create Maximum Number
pub struct Solution;
impl Solution {
    fn max_number_iner(
        nums1: &[i32],
        nums2: &[i32],
        k: usize,
        cache: &mut std::collections::HashMap<(usize, usize, usize), Vec<i32>>,
    ) -> Vec<i32> {
        if let Some(v) = cache.get(&(nums1.len(), nums2.len(), k)) {
            return v.clone();
        }
        let mut res = Vec::new();
        let n = nums1.len() + nums2.len() + 1 - k;
        let mut maxn = -1;
        let mut maxi1 = usize::MAX;
        let mut maxi2 = usize::MAX;
        for i in 0..n.min(nums1.len()) {
            if nums1[i] > maxn {
                maxn = nums1[i];
                maxi1 = i;
            }
        }
        for i in 0..n.min(nums2.len()) {
            if nums2[i] > maxn {
                maxn = nums2[i];
                maxi2 = i;
                maxi1 = usize::MAX;
            } else if nums2[i] == maxn && maxi2 == usize::MAX {
                maxi2 = i;
            }
        }
        if k > 1 {
            let mut res1 = vec![];
            if maxi1 != usize::MAX {
                res1 = Self::max_number_iner(&nums1[(maxi1 + 1)..], nums2, k - 1, cache);
            }
            let mut res2 = vec![];
            if maxi2 != usize::MAX {
                res2 = Self::max_number_iner(nums1, &nums2[(maxi2 + 1)..], k - 1, cache);
            }
            let mut use1 = !res1.is_empty();
            if use1 && !res2.is_empty() {
                for i in (0..res1.len()).rev() {
                    if res1[i] > res2[i] {
                        break;
                    }
                    if res1[i] < res2[i] {
                        use1 = false;
                        break;
                    }
                }
            }
            if use1 {
                res = res1;
            } else {
                res = res2;
            }
        }
        res.push(maxn);
        cache.insert((nums1.len(), nums2.len(), k), res.clone());
        res
    }
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        Self::max_number_iner(
            &nums1[..],
            &nums2[..],
            k as usize,
            &mut std::collections::HashMap::new(),
        )
        .into_iter()
        .rev()
        .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_number() {
        assert_eq!(
            Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
        assert_eq!(
            Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );
        assert_eq!(
            Solution::max_number(vec![3, 9], vec![8, 9], 3),
            vec![9, 8, 9]
        );
    }
}
