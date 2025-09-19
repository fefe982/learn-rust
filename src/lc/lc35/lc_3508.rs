// https://leetcode.com/problems/implement-router/
// 3508. Implement Router
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
pub struct Router {
    memory_limit: usize,
    packets: VecDeque<(i32, i32, i32)>,
    counts: HashMap<i32, i32>,
    destinations: HashMap<i32, BTreeMap<i32, (i32, HashSet<i32>)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    pub fn new(memory_limit: i32) -> Self {
        Self {
            memory_limit: memory_limit as usize,
            packets: VecDeque::new(),
            counts: HashMap::new(),
            destinations: HashMap::new(),
        }
    }

    pub fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        if let Some(d) = self.destinations.get(&destination) {
            if let Some((_, s)) = d.get(&timestamp) {
                if s.contains(&source) {
                    return false;
                }
            }
        }
        if self.packets.len() >= self.memory_limit {
            self.forward_packet();
        }
        self.packets.push_back((source, destination, timestamp));
        let dc = self.counts.entry(destination).or_insert(0);
        *dc += 1;
        self.destinations
            .entry(destination)
            .or_default()
            .entry(timestamp)
            .and_modify(|(c, s)| {
                *c += 1;
                s.insert(source);
            })
            .or_insert((*dc, HashSet::from([source])));
        true
    }

    pub fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((source, destination, timestamp)) = self.packets.pop_front() {
            if let Some(d) = self.destinations.get_mut(&destination) {
                if let Some((_, s)) = d.get_mut(&timestamp) {
                    s.remove(&source);
                    if s.is_empty() {
                        d.remove(&timestamp);
                    }
                }
                if d.is_empty() {
                    self.destinations.remove(&destination);
                }
            }
            vec![source, destination, timestamp]
        } else {
            vec![]
        }
    }

    pub fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(d) = self.destinations.get(&destination) {
            if let Some((_, s)) = d.range(start_time..=end_time).next() {
                let dc = s.0;
                let cc = s.1.len() as i32;
                if let Some((_, r)) = d.range(..=end_time).next_back() {
                    r.0 - dc + cc
                } else {
                    cc
                }
            } else {
                0
            }
        } else {
            0
        }
    }
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_3508() {
        let null = 0;
        for test in vec![
            (
                vec![
                    "Router",
                    "addPacket",
                    "addPacket",
                    "addPacket",
                    "getCount",
                    "forwardPacket",
                    "getCount",
                ],
                vec_any![[2], [1, 4, 1], [5, 4, 1], [1, 4, 1], [4, 1, 1], [], [4, 1, 1]],
                vec_any![null, true, true, false, 2, [1, 4, 1], 1],
            ),
            (
                vec![
                    "Router",
                    "addPacket",
                    "addPacket",
                    "addPacket",
                    "addPacket",
                    "addPacket",
                    "forwardPacket",
                    "addPacket",
                    "getCount",
                ],
                vec_any![
                    [3],
                    [1, 4, 90],
                    [2, 5, 90],
                    [1, 4, 90],
                    [3, 5, 95],
                    [4, 5, 105],
                    [],
                    [5, 2, 110],
                    [5, 100, 110]
                ],
                vec_any![null, true, true, false, true, true, [2, 5, 90], true, 1],
            ),
            (
                vec!["Router", "addPacket", "forwardPacket", "forwardPacket"],
                vec_any![[2], [7, 4, 90], [], []],
                vec_any![null, true, [7, 4, 90], []],
            ),
        ] {
            let mut obj = Router::new(test.1[0][0].as_i32());
            for ((&cmd, args), expect) in test.0.iter().zip(test.1.iter()).zip(test.2.iter()).skip(1) {
                match cmd {
                    "addPacket" => assert_eq!(
                        obj.add_packet(args[0].as_i32(), args[1].as_i32(), args[2].as_i32()),
                        expect.as_bool()
                    ),
                    "forwardPacket" => assert_eq!(obj.forward_packet(), expect.as_vec_i32()),
                    "getCount" => assert_eq!(
                        obj.get_count(args[0].as_i32(), args[1].as_i32(), args[2].as_i32()),
                        expect.as_i32()
                    ),
                    _ => {}
                }
            }
        }
    }
}
