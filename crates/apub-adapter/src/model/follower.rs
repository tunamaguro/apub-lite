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
    pub actor_url: String,
}

impl TryFrom<FollowerRow> for Follower {
    type Error = anyhow::Error;
    fn try_from(value: FollowerRow) -> Result<Self, Self::Error> {
        let FollowerRow {
            user_id, actor_url, ..
        } = value;

        let user_id = user_id.into();
        let actor_url = actor_url.parse::<ResourceUrl>()?;

        Ok(Follower { user_id, actor_url })
    }
}
