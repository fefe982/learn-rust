// https://leetcode.com/problems/random-flip-matrix/
// 519. Random Flip Matrix
use rand::Rng;
pub struct Solution {
    m: i32,
    n: i32,
    zpos: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(m: i32, n: i32) -> Self {
        Self { m, n, zpos: Vec::new() }
    }

    pub fn flip(&mut self) -> Vec<i32> {
        let len = self.m * self.n - self.zpos.len() as i32;
        let mut target = rand::thread_rng().gen_range(0..len);
        let mut pos = self.zpos.partition_point(|&x| x <= target);
        target += pos as i32;
        while pos < self.zpos.len() && self.zpos[pos] <= target {
            target += 1;
            pos += 1;
        }
        self.zpos.insert(pos, target);
        vec![target / self.n, target % self.n]
    }

    pub fn reset(&mut self) {
        self.zpos.clear();
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(m, n);
 * let ret_1: Vec<i32> = obj.flip();
 * obj.reset();
 */
#[cfg(test)]
mod tests {
    use super::*;
    fn flip(v: &mut Vec<Vec<i32>>, p: Vec<i32>) {
        let i = p[0] as usize;
        let j = p[1] as usize;
        assert!(v[i][j] == 0);
        v[i][j] = 1;
    }
    #[test]
    fn test_519() {
        let n = 3;
        let m = 2;
        let mut obj = Solution::new(2, 3);
        let mut v = vec![vec![0; n as usize]; m as usize];
        flip(&mut v, obj.flip());
        flip(&mut v, obj.flip());
        flip(&mut v, obj.flip());
        obj.reset();
        v = vec![vec![0; n as usize]; m as usize];
        flip(&mut v, obj.flip());
    }
}
