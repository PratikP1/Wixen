// Mastodon models
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MastodonAccount {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub created_at: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub header: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MastodonStatus {
    pub id: String,
    pub created_at: String,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: String,
    pub language: Option<String>,
    pub uri: String,
    pub url: Option<String>,
    pub replies_count: u32,
    pub reblogs_count: u32,
    pub favourites_count: u32,
    pub content: String,
    pub account: MastodonAccount,
}

#[derive(Debug, Clone)]
pub struct MastodonTimeline {
    pub statuses: Vec<MastodonStatus>,
}
