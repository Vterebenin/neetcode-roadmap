use crate::utils::print_pass;
use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse};

const NAME: &str = "last-stone-weight";



static MAX_ITEMS: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Post {
    id: i32,
    post_counter: i32
}
#[derive(Debug, Clone)]
struct User {
    followers: Vec<i32>,
    posts: Vec<Post>,
}
#[derive(Debug, Clone)]
struct Twitter {
    users: HashMap<i32, User>,
    posts_counter: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        Twitter {
            users: HashMap::new(),
            posts_counter: 0,
        }
    }
    
    fn create_user_or_none(&mut self, user_id: i32) {
        self.users
            .entry(user_id)
            .or_insert(User { 
                followers: vec![], 
                posts: vec![],
            });
    }
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.posts_counter += 1;
        let post = Post {
           id: tweet_id,
           post_counter: self.posts_counter,
        };
        self.users
            .entry(user_id)
            .and_modify(|user| {
                user.posts.push(post)
            })
            .or_insert(User { 
                followers: vec![], 
                posts: vec![post],
            });
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        if let Some(user) = self.users.get(&user_id) {
            let mut heap = BinaryHeap::with_capacity(MAX_ITEMS);
            for post in user.posts.iter() {
                heap.push(Reverse((post.post_counter, post.id)));
                if heap.len() > MAX_ITEMS {
                    heap.pop();
                }
            }
            for follower_id in user.followers.iter() {
                if let Some(follower) = self.users.get(&follower_id) {
                    for post in follower.posts.iter() {
                        heap.push(Reverse((post.post_counter, post.id)));
                        if heap.len() > MAX_ITEMS {
                            heap.pop();
                        }
                    }
                }
            }
            let mut answer = vec![];
            while !heap.is_empty() {
                answer.push(heap.pop().unwrap().0.1)
            }
            answer.reverse();
            return answer;
        }
        vec![]
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.create_user_or_none(follower_id);
        if let Some(follower) = self.users.get_mut(&follower_id) {
            if !follower.followers.contains(&followee_id) {
                follower.followers.push(followee_id);
            }
        }
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(follower) = self.users.get_mut(&follower_id) {
            if let Some(index) = follower.followers.iter().position(|f| *f == followee_id) {
                follower.followers.remove(index);
            };
        }
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

pub fn main() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 3); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 101); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 13); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 10); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 2); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 94); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 505); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 333); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 22); // User 1 posts a new tweet (id = 5).
    twitter.post_tweet(1, 11); // User 1 posts a new tweet (id = 5).
    twitter.get_news_feed(1); // User 1 posts a new tweet (id = 5).
    twitter.follow(1, 2);  // User 1 unfollows user 2.
    twitter.get_news_feed(1);  // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
    twitter.post_tweet(2, 6); // User 2 posts a new tweet (id = 6).
    twitter.get_news_feed(1);  // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
    twitter.unfollow(1, 2);  // User 1 unfollows user 2.
    twitter.get_news_feed(1);  // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
    print_pass(NAME);
}
