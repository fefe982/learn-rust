// https://leetcode.com/problems/frequency-tracker/
// 2671. Frequency Tracker
pub struct FrequencyTracker {
    num_map: std::collections::HashMap<i32, i32>,
    freq_map: std::collections::HashMap<i32, std::collections::HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {
    pub fn new() -> Self {
        Self {
            num_map: std::collections::HashMap::new(),
            freq_map: std::collections::HashMap::new(),
        }
    }

    pub fn add(&mut self, number: i32) {
        let freq = self.num_map.entry(number).or_insert(0);
        if *freq > 0 {
            let set = self.freq_map.entry(*freq).or_default();
            set.remove(&number);
            if set.is_empty() {
                self.freq_map.remove(freq);
            }
        }
        *freq += 1;
        self.freq_map.entry(*freq).or_default().insert(number);
    }

    pub fn delete_one(&mut self, number: i32) {
        if let Some(freq) = self.num_map.get_mut(&number) {
            let set = self.freq_map.entry(*freq).or_default();
            set.remove(&number);
            if set.is_empty() {
                self.freq_map.remove(freq);
            }
            *freq -= 1;
            if *freq > 0 {
                self.freq_map.entry(*freq).or_default().insert(number);
            } else {
                self.num_map.remove(&number);
            }
        }
    }

    pub fn has_frequency(&mut self, frequency: i32) -> bool {
        self.freq_map.contains_key(&frequency)
    }
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */
#[cfg(test)]
mod tests {
    use super::*;
    enum Call {
        Add(i32),
        DeleteOne(i32),
        HasFrequency(i32, bool),
    }
    #[test]
    fn test_2671() {
        for test in [
            vec![
                Call::DeleteOne(9),
                Call::DeleteOne(8),
                Call::DeleteOne(1),
                Call::DeleteOne(4),
                Call::HasFrequency(1, false),
                Call::Add(10),
                Call::DeleteOne(5),
                Call::HasFrequency(1, true),
                Call::DeleteOne(10),
                Call::DeleteOne(9),
                Call::DeleteOne(10),
                Call::HasFrequency(1, false),
                Call::Add(4),
                Call::HasFrequency(1, true),
                Call::DeleteOne(4),
                Call::HasFrequency(1, false),
                Call::HasFrequency(1, false),
                Call::Add(10),
                Call::HasFrequency(1, true),
            ],
            vec![Call::Add(3), Call::Add(3), Call::HasFrequency(2, true)],
            vec![Call::Add(1), Call::DeleteOne(1), Call::HasFrequency(1, false)],
            vec![Call::HasFrequency(2, false), Call::Add(3), Call::HasFrequency(1, true)],
        ] {
            let mut obj = FrequencyTracker::new();
            for call in test {
                match call {
                    Call::Add(number) => obj.add(number),
                    Call::DeleteOne(number) => obj.delete_one(number),
                    Call::HasFrequency(frequency, expect) => {
                        assert_eq!(obj.has_frequency(frequency), expect)
                    }
                }
            }
        }
    }
}
