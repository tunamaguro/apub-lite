use apub_activitypub::webfinger::AcctUri;
use apub_kernel::follower::model::Follower;
use apub_shared::model::resource_url::ResourceUrl;
use sqlx::types::Uuid;

pub struct FollowerCount {
    pub count: Option<i64>,
}

impl FollowerCount {
    pub fn count(&self) -> i64 {
        self.count.unwrap_or(0)
    }
}

pub struct FollowerRow {
    pub user_id: Uuid,
    pub follower_url: String,
    pub host: String,
    pub preferred_username: String,
    pub inbox_url: String,
}

impl TryFrom<FollowerRow> for Follower {
    type Error = anyhow::Error;
    fn try_from(value: FollowerRow) -> Result<Self, Self::Error> {
        let FollowerRow {
            user_id,
            follower_url,
            host,
            inbox_url,
            preferred_username,
            ..
        } = value;

        let user_id = user_id.into();
        let actor_url = follower_url.parse::<ResourceUrl>()?;
        let inbox_url = inbox_url.parse::<ResourceUrl>()?;
        let acct = AcctUri::new(host, preferred_username)?;

        let follower = Follower::builder()
            .acct(acct)
            .inbox(inbox_url)
            .actor_url(actor_url)
            .user_id(user_id)
            .build();
        Ok(follower)
    }
}
