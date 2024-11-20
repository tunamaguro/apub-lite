use serde::Serialize;

use crate::model::activity::SendActivity;

#[async_trait::async_trait]
pub trait ActivityRepository: Send + Sync {
    /// 固定の`Create``Note`を`inbox`に送るだけ
    async fn send_create_activity<T: Serialize + Sync>(
        &self,
        event: &SendActivity<T>,
    ) -> anyhow::Result<()>;
}
