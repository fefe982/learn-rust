// https://leetcode.com/problems/online-election/
// 911. Online Election
pub struct TopVotedCandidate {
    times: Vec<i32>,
    winner: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    pub fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut winner = Vec::with_capacity(times.len());
        let mut count = vec![0; 5001];
        let mut max = 0;
        for p in persons {
            count[p as usize] += 1;
            if count[p as usize] >= count[max as usize] {
                max = p;
            }
            winner.push(max);
        }
        Self { times, winner }
    }

    pub fn q(&self, t: i32) -> i32 {
        let i = self.times.partition_point(|&x| x <= t);
        self.winner[i - 1]
    }
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_911() {
        let obj = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
        assert_eq!(obj.q(3), 0);
        assert_eq!(obj.q(12), 1);
        assert_eq!(obj.q(25), 1);
        assert_eq!(obj.q(15), 0);
        assert_eq!(obj.q(24), 0);
        assert_eq!(obj.q(8), 1);
    }
}
