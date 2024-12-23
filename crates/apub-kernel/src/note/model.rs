use apub_config::AppConfig;
use apub_shared::model::id::{Id, UrlId};

use crate::user::model::UserId;

pub type NoteId = Id<Note>;
pub type NoteUrl = UrlId<Note>;

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub id: NoteId,
    pub user_id: UserId,
    pub content: String,
}

impl Note {
    pub fn note_uri(&self, config: &AppConfig) -> NoteUrl {
        let note_uri = config
            .host_uri()
            .clone()
            .set_path(&format!("/notes/{}", self.id))
            .to_owned();
        note_uri.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateNote {
    pub note_id: NoteId,
    pub user_id: UserId,
    pub content: String,
}

impl CreateNote {
    pub fn new(user_id: UserId, content: String) -> Self {
        let note_id = NoteId::new();
        Self {
            note_id,
            user_id,
            content,
        }
    }
}
