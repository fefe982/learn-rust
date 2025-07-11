// https://leetcode.com/problems/count-ways-to-build-rooms-in-an-ant-colony/
// 1916. Count Ways to Build Rooms in an Ant Colony
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn div(mut a: i64, mut b: i64) -> i64 {
        while a % b != 0 {
            let t = MOD / b + 1;
            b = b * t - MOD;
            a = a * t % MOD;
        }
        a / b
    }
    fn comb(a: i32, b: i32, fac: &Vec<i64>) -> i64 {
        let a = a as usize;
        let b = b as usize;
        Self::div(Self::div(fac[a + b], fac[a]), fac[b])
    }
    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        let n = prev_room.len();
        let mut fac = vec![1; n + 1];
        for i in 2..=n {
            fac[i] = fac[i - 1] * i as i64 % MOD;
        }
        let mut cnt = vec![0; n];
        for &p in &prev_room {
            if p >= 0 {
                cnt[p as usize] += 1;
            }
        }
        let mut s = vec![];
        let mut ways = vec![(0, 0); n];
        for (i, &c) in cnt.iter().enumerate() {
            if c == 0 {
                s.push(i);
                ways[i] = (1, 1);
            }
        }
        while let Some(i) = s.pop() {
            let p = prev_room[i];
            if p < 0 {
                break;
            }
            let p = p as usize;
            if ways[p].1 == 0 {
                ways[p] = ways[i];
            } else {
                ways[p].0 = (ways[p].0 * ways[i].0) % MOD * Self::comb(ways[p].1, ways[i].1, &fac) % MOD;
                ways[p].1 += ways[i].1;
            }
            cnt[p] -= 1;
            if cnt[p] == 0 {
                ways[p].1 += 1;
                s.push(p);
            }
        }
        ways[0].0 as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ways_to_build_rooms() {
        assert_eq!(Solution::ways_to_build_rooms(vec![-1, 0, 1]), 1);
        assert_eq!(Solution::ways_to_build_rooms(vec![-1, 0, 0, 1, 2]), 6);
    }
}
