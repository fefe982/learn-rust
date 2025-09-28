// https://leetcode.com/problems/tweet-counts-per-frequency/
// 1348. Tweet Counts Per Frequency
pub struct TweetCounts {
    tweets: std::collections::HashMap<String, std::collections::BTreeMap<i32, i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TweetCounts {
    pub fn new() -> Self {
        Self {
            tweets: std::collections::HashMap::new(),
        }
    }

    pub fn record_tweet(&mut self, tweet_name: String, time: i32) {
        self.tweets
            .entry(tweet_name)
            .or_default()
            .entry(time)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    pub fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let delta = match freq.as_str() {
            "minute" => 60,
            "hour" => 3600,
            "day" => 86400,
            _ => unreachable!(),
        };
        let mut ans = vec![0; ((end_time - start_time) / delta + 1) as usize];
        if let Some(map) = self.tweets.get(&tweet_name) {
            for (&t, &c) in map.range(start_time..=end_time) {
                ans[((t - start_time) / delta) as usize] += c;
            }
        }
        ans
    }
}

/**
 * Your TweetCounts object will be instantiated and called as such:
 * let obj = TweetCounts::new();
 * obj.record_tweet(tweetName, time);
 * let ret_2: Vec<i32> = obj.get_tweet_counts_per_frequency(freq, tweetName, startTime, endTime);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tweet_counts() {
        let mut obj = TweetCounts::new();
        obj.record_tweet("tweet3".to_string(), 0);
        obj.record_tweet("tweet3".to_string(), 60);
        obj.record_tweet("tweet3".to_string(), 10);
        assert_eq!(
            obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 59),
            vec![2]
        );
        assert_eq!(
            obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 60),
            vec![2, 1]
        );
        obj.record_tweet("tweet3".to_string(), 120);
        assert_eq!(
            obj.get_tweet_counts_per_frequency("hour".to_string(), "tweet3".to_string(), 0, 210),
            vec![4]
        );
    }
}
