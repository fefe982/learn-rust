// https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/
// 497. Random Point in Non-overlapping Rectangles
use rand::distributions::Distribution;
struct Solution {
    rects: Vec<Vec<i32>>,
    start: Vec<i32>,
    uni: rand::distributions::Uniform<i32>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut start = Vec::with_capacity(rects.len());
        let mut total = 0;
        for r in &rects {
            start.push(total);
            total += (r[2] - r[0] + 1) * (r[3] - r[1] + 1);
        }
        Self {
            rects,
            start,
            uni: rand::distributions::Uniform::new(0, total),
            rng: rand::thread_rng(),
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let val = self.uni.sample(&mut self.rng);
        let pos = self.start.partition_point(|&x| x <= val) - 1;
        let offset = val - self.start[pos];
        let x = offset % (self.rects[pos][2] - self.rects[pos][0] + 1) + self.rects[pos][0];
        let y = offset / (self.rects[pos][2] - self.rects[pos][0] + 1) + self.rects[pos][1];
        vec![x, y]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_497() {
        let v = vec_vec![[-2, -2, 1, 1], [2, 2, 4, 6]];
        let mut s = Solution::new(v.clone());
        for _ in 0..5 {
            let p = s.pick();
            let mut inside = false;
            for r in &v {
                if p[0] >= r[0] && p[0] <= r[2] && p[1] >= r[1] && p[1] <= r[3] {
                    inside = true;
                    break;
                }
            }
            assert!(inside);
        }
    }
}
