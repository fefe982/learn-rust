// https://leetcode.com/problems/number-of-music-playlists/
// 920. Number of Music Playlists
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i32) -> Self {
        Self { i: i as i64 }
    }
    fn from_i64(i: i64) -> Self {
        Self { i }
    }
    fn val(self) -> i32 {
        self.i as i32
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = IMod;
    fn add(self, rhs: IMod) -> IMod {
        IMod::from_i64((self.i + rhs.i) % MOD)
    }
}
impl std::ops::Sub<IMod> for IMod {
    type Output = IMod;
    fn sub(self, rhs: IMod) -> Self::Output {
        IMod::from_i64((self.i + MOD - rhs.i) % MOD)
    }
}
impl std::ops::AddAssign<IMod> for IMod {
    fn add_assign(&mut self, rhs: IMod) {
        self.i = (self.i + rhs.i) % MOD;
    }
}
impl std::ops::Mul<i32> for IMod {
    type Output = IMod;
    fn mul(self, rhs: i32) -> Self::Output {
        IMod::from_i64((self.i * rhs as i64) % MOD)
    }
}
impl std::ops::MulAssign<i32> for IMod {
    fn mul_assign(&mut self, rhs: i32) {
        self.i = (self.i * rhs as i64) % MOD;
    }
}
impl std::ops::MulAssign<IMod> for IMod {
    fn mul_assign(&mut self, rhs: IMod) {
        self.i = (self.i * rhs.i) % MOD;
    }
}
impl Solution {
    fn num_playlist(n: i32, goal: i32, k: i32, cache: &mut Vec<Vec<IMod>>) -> IMod {
        if n == 0 && goal == 0 {
            return IMod::new(1);
        }
        if n == 0 || goal == 0 {
            return IMod::new(0);
        }
        if cache[n as usize - 1][goal as usize - 1].val() != -1 {
            return cache[n as usize - 1][goal as usize - 1];
        }
        let mut cnt = Self::num_playlist(n - 1, goal - 1, k, cache) * n;
        if n > k {
            cnt += Self::num_playlist(n, goal - 1, k, cache) * (n - k);
        }
        cache[n as usize - 1][goal as usize - 1] = cnt;
        cnt
    }
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        Self::num_playlist(
            n,
            goal,
            k,
            &mut vec![vec![IMod::new(-1); goal as usize]; n as usize],
        )
        .val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_music_playlists() {
        assert_eq!(Solution::num_music_playlists(3, 3, 0), 6);
        assert_eq!(Solution::num_music_playlists(3, 3, 1), 6);
        assert_eq!(Solution::num_music_playlists(2, 3, 0), 6);
        assert_eq!(Solution::num_music_playlists(2, 3, 1), 2);
    }
}
