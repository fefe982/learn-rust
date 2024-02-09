// https://leetcode.com/problems/dice-roll-simulation/
// 1223. Dice Roll Simulation
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
#[derive(Copy, Clone, Debug)]
struct IMod {
    v: i64,
}
impl Default for IMod {
    fn default() -> Self {
        Self { v: 0 }
    }
}
impl IMod {
    fn from_i32(v: i32) -> Self {
        Self { v: v as i64 }
    }
    fn to_i32(&self) -> i32 {
        self.v as i32
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, rhs: Self) {
        self.v = (self.v + rhs.v) % MOD;
    }
}
impl std::ops::Add<&IMod> for IMod {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        Self {
            v: (self.v + rhs.v) % MOD,
        }
    }
}
impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let mut state_map = std::collections::HashMap::new();
        state_map.insert((0, 0), IMod::from_i32(1));
        for _ in 0..n {
            let mut next_state_map = std::collections::HashMap::new();
            for ((dice, roll), tot) in state_map {
                for i in 0..6 {
                    if i == dice {
                        if roll < roll_max[i] {
                            *next_state_map.entry((i, roll + 1)).or_default() += tot;
                        }
                    } else {
                        *next_state_map.entry((i, 1)).or_default() += tot;
                    }
                }
            }
            state_map = next_state_map;
        }
        state_map.values().fold(IMod::from_i32(0), |acc, x| acc + x).to_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_die_simulator() {
        assert_eq!(Solution::die_simulator(2, vec![1, 1, 2, 2, 2, 3]), 34);
        assert_eq!(Solution::die_simulator(2, vec![1, 1, 1, 1, 1, 1]), 30);
        assert_eq!(Solution::die_simulator(3, vec![1, 1, 1, 2, 2, 3]), 181);
    }
}
