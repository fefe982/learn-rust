// https://leetcode.com/problems/design-twitter/
// 355. Design Twitter
pub struct Twitter {
    tweets: std::collections::HashMap<i32, Vec<(i32, i32)>>,
    follows: std::collections::HashMap<i32, std::collections::HashSet<i32>>,
    time: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    pub fn new() -> Self {
        Self {
            tweets: std::collections::HashMap::new(),
            follows: std::collections::HashMap::new(),
            time: 0,
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.entry(user_id).or_insert(vec![]).push((self.time, tweet_id));
        self.time += 1;
    }

    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut h = std::collections::BinaryHeap::new();
        for user in self
            .follows
            .get(&user_id)
            .unwrap_or(&std::collections::HashSet::new())
            .iter()
            .chain(std::iter::once(&user_id))
        {
            for &tweet in self.tweets.get(user).unwrap_or(&vec![]).iter().rev().take(10) {
                h.push(tweet);
            }
        }
        let mut res = vec![];
        while let Some((_, tweet_id)) = h.pop() {
            res.push(tweet_id);
            if res.len() == 10 {
                break;
            }
        }
        res
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows
            .entry(follower_id)
            .or_insert(std::collections::HashSet::new())
            .insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).and_modify(|s| {
            s.remove(&followee_id);
        });
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_twitter() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
    }
}
