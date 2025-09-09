// Bluesky models
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BlueskyAccount {
    pub did: String,
    pub handle: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub indexed_at: Option<String>,
    pub followers_count: Option<u32>,
    pub follows_count: Option<u32>,
    pub posts_count: Option<u32>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BlueskyPost {
    pub uri: String,
    pub cid: String,
    pub author: BlueskyAccount,
    pub text: String,
    pub created_at: String,
    pub reply_count: Option<u32>,
    pub repost_count: Option<u32>,
    pub like_count: Option<u32>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BlueskyFeedType {
    Timeline,
    Notifications,
    Custom(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BlueskyFeed {
    pub feed_type: BlueskyFeedType,
    pub posts: Vec<BlueskyPost>,
}
