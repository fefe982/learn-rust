// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-ii/
// 3321. Find X-Sum of All K-Long Subarrays II
pub struct Solution;
#[derive(Debug)]
struct FrequencyTracker {
    x: i32,
    cnt: i32,
    fbound: i32,
    nbound: i32,
    sum: i64,
    num_map: std::collections::HashMap<i32, i32>,
    freq_map: std::collections::BTreeMap<i32, std::collections::BTreeSet<i32>>,
}

impl FrequencyTracker {
    fn new(x: i32) -> Self {
        Self {
            x,
            cnt: 0,
            fbound: 1,
            nbound: 0,
            sum: 0,
            num_map: std::collections::HashMap::new(),
            freq_map: std::collections::BTreeMap::new(),
        }
    }

    fn add(&mut self, number: i32) -> i64 {
        let freq = self.num_map.entry(number).or_insert(0);
        if *freq > 0 {
            let set = self.freq_map.entry(*freq).or_default();
            set.remove(&number);
            if set.is_empty() {
                self.freq_map.remove(freq);
            }
        }
        if *freq > self.fbound || (*freq == self.fbound && number >= self.nbound) {
            self.sum += number as i64;
        } else if *freq + 1 > self.fbound || (*freq + 1 == self.fbound && number >= self.nbound) {
            self.sum += number as i64 * (*freq as i64 + 1);
            self.cnt += 1;
        }
        *freq += 1;
        self.freq_map.entry(*freq).or_default().insert(number);
        self.get_xsum()
    }

    fn delete_one(&mut self, number: i32) {
        if let Some(freq) = self.num_map.get_mut(&number) {
            let set = self.freq_map.entry(*freq).or_default();
            set.remove(&number);
            if set.is_empty() {
                self.freq_map.remove(freq);
            }
            *freq -= 1;
            if *freq > self.fbound || (*freq == self.fbound && number >= self.nbound) {
                self.sum -= number as i64;
            } else if *freq + 1 > self.fbound || (*freq + 1 == self.fbound && number >= self.nbound) {
                self.sum -= number as i64 * (*freq as i64 + 1);
                self.cnt -= 1;
            }
            if *freq > 0 {
                self.freq_map.entry(*freq).or_default().insert(number);
            } else {
                self.num_map.remove(&number);
            }
        }
    }

    fn get_xsum(&mut self) -> i64 {
        if self.cnt > self.x {
            for (&freq, set) in self.freq_map.range_mut(self.fbound..) {
                if freq > self.fbound {
                    self.fbound = freq;
                    self.nbound = 0;
                }
                for &n in set.range(self.nbound..) {
                    self.sum -= freq as i64 * n as i64;
                    self.cnt -= 1;
                    self.nbound = n + 1;
                    if self.cnt == self.x {
                        return self.sum;
                    }
                }
            }
        }
        if self.cnt < self.x {
            for (&freq, set) in self.freq_map.range_mut(..=self.fbound).rev() {
                if freq < self.fbound {
                    self.fbound = freq;
                    self.nbound = i32::MAX;
                }
                for &n in set.range(..self.nbound).rev() {
                    self.sum += freq as i64 * n as i64;
                    self.cnt += 1;
                    self.nbound = n;
                    if self.cnt == self.x {
                        return self.sum;
                    }
                }
            }
        }
        self.sum
    }
}
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let mut tracker = FrequencyTracker::new(x);
        let k = k as usize;
        for i in 0..k - 1 {
            tracker.add(nums[i]);
        }
        let mut result = Vec::with_capacity(nums.len() - k + 1);
        for i in k - 1..nums.len() {
            result.push(tracker.add(nums[i]));
            tracker.delete_one(nums[i + 1 - k]);
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_x_sum() {
        assert_eq!(
            Solution::find_x_sum(vec![3, 1, 1, 2, 2, 6, 3, 5, 6], 4, 1),
            vec![2, 4, 4, 4, 6, 12]
        );
        assert_eq!(
            Solution::find_x_sum(vec![4, 4, 3, 5, 5, 6, 5, 3, 3, 3], 7, 1),
            vec![15, 15, 15, 15]
        );
        assert_eq!(
            Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
            vec![6, 10, 12]
        );
        assert_eq!(
            Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
            vec![11, 15, 15, 15, 12]
        );
    }
}
