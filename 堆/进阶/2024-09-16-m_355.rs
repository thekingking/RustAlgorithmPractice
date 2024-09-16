struct Solution;

struct Twitter {
  follow: Vec<Vec<bool>>,
  tweet: Vec<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
      Self {
        follow: vec![vec![false; 501]; 501],
        tweet: Vec::new(),
      }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
      self.tweet.push((user_id, tweet_id));
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
      let mut ans = Vec::new();
      for &(id, tweet_id) in self.tweet.iter().rev() {
        if user_id == id || self.follow[user_id as usize][id as usize] {
          ans.push(tweet_id);
        }
        if ans.len() >= 10 {
          break;
        }
      }
      ans
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
      self.follow[follower_id as usize][followee_id as usize] = true;
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
      self.follow[follower_id as usize][followee_id as usize] = false;
    }
}