use crate::user::model::UserId;

use super::model::{CreateNote, Note, NoteId};

#[async_trait::async_trait]
pub trait NoteRepository: Send + Sync {
    async fn find(&self, note_id: &NoteId) -> anyhow::Result<Note>;
    async fn list_user_notes(&self, user_id: &UserId) -> anyhow::Result<Vec<Note>>;
    async fn create(&self, event: &CreateNote) -> anyhow::Result<()>;
    async fn delete(&self, note_id: &NoteId) -> anyhow::Result<()>;
}
