use apub_kernel::note::model::Note;
use sqlx::types::Uuid;

pub struct NoteRow {
    pub note_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
}

impl From<NoteRow> for Note {
    fn from(value: NoteRow) -> Self {
        let NoteRow {
            note_id,
            user_id,
            content,
        } = value;

        Note {
            id: note_id.into(),
            user_id: user_id.into(),
            content,
        }
    }
}
