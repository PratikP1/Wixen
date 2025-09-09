// RSS models
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub pub_date: Option<String>,
    pub guid: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RssChannel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub language: Option<String>,
    pub items: Vec<RssItem>,
}

#[derive(Debug, Clone)]
pub struct RssFeed {
    pub channel: RssChannel,
}
