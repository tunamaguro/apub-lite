use apub_kernel::{
    note::{
        model::{CreateNote, Note, NoteId},
        repository::NoteRepository,
    },
    user::model::UserId,
};

use crate::{model::note::NoteRow, persistence::postgres::PostgresDb};

#[async_trait::async_trait]
impl NoteRepository for PostgresDb {
    #[tracing::instrument(skip(self))]
    async fn find(&self, note_id: &NoteId) -> anyhow::Result<Note> {
        let row = sqlx::query_as!(
            NoteRow,
            r#"
            SELECT 
                note_id, user_id, content
            FROM
                notes
            WHERE
                notes.note_id = $1
        "#,
            note_id.as_ref()
        )
        .fetch_one(self.inner_ref())
        .await?;

        Ok(row.into())
    }

    #[tracing::instrument(skip(self))]
    async fn list_user_notes(&self, user_id: &UserId) -> anyhow::Result<Vec<Note>> {
        let rows = sqlx::query_as!(
            NoteRow,
            r#"
            SELECT 
                note_id, user_id, content
            FROM
                notes
            WHERE
                notes.user_id = $1
        "#,
            user_id.as_ref()
        )
        .fetch_all(self.inner_ref())
        .await?;

        let notes = rows.into_iter().map(|r| r.into()).collect();

        Ok(notes)
    }

    #[tracing::instrument(skip_all)]
    async fn create(&self, event: &CreateNote) -> anyhow::Result<()> {
        let _count = sqlx::query!(
            r#"
            INSERT INTO notes (note_id, user_id, content)
            VALUES ($1,$2,$3)
        "#,
            event.note_id.as_ref(),
            event.user_id.as_ref(),
            event.content
        )
        .execute(self.inner_ref())
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn delete(&self, note_id: &NoteId) -> anyhow::Result<()> {
        let count = sqlx::query!(
            r#"
            DELETE FROM notes
            WHERE
                notes.note_id = $1
        "#,
            note_id.as_ref(),
        )
        .execute(self.inner_ref())
        .await?;

        if count.rows_affected() != 1 {
            Err(anyhow::anyhow!("No rows deleted"))
        } else {
            Ok(())
        }
    }
}
