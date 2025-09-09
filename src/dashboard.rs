// dashboard.rs
// Handles rendering and column management for the multi-service dashboard

use crate::mastodon::MastodonTimeline;
use crate::bluesky::BlueskyFeed;
use crate::rss::RssFeed;

pub enum DashboardColumn {
    Mastodon(MastodonTimeline),
    Bluesky(BlueskyFeed),
    Rss(RssFeed),
}

pub struct Dashboard {
    pub columns: Vec<DashboardColumn>,
}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard { columns: Vec::new() }
    }

    pub fn add_mastodon_column(&mut self, timeline: MastodonTimeline) {
        self.columns.push(DashboardColumn::Mastodon(timeline));
    }

    pub fn add_bluesky_column(&mut self, feed: BlueskyFeed) {
        self.columns.push(DashboardColumn::Bluesky(feed));
    }

    pub fn add_rss_column(&mut self, feed: RssFeed) {
        self.columns.push(DashboardColumn::Rss(feed));
    }

    pub fn render(&self) {
        // Example rendering logic (replace with actual UI code)
        for (i, col) in self.columns.iter().enumerate() {
            match col {
                DashboardColumn::Mastodon(timeline) => {
                    println!("Column {}: Mastodon Timeline ({} statuses)", i + 1, timeline.statuses.len());
                }
                DashboardColumn::Bluesky(feed) => {
                    println!("Column {}: Bluesky Feed ({} posts)", i + 1, feed.posts.len());
                }
                DashboardColumn::Rss(feed) => {
                    println!("Column {}: RSS Feed ({} items)", i + 1, feed.channel.items.len());
                }
            }
        }
    }
}
