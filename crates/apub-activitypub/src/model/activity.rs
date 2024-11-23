use apub_shared::model::id::UrlId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use super::{context::Context, note::Note, person::Person, SingleOrMany};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CreateKind {
    #[default]
    Create,
}

/// Create activity
///
/// See https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Create<T> {
    #[serde(rename = "@context")]
    context: Context,
    id: UrlId<Create<T>>,
    #[serde(rename = "type")]
    #[builder(default)]
    kind: CreateKind,
    actor: UrlId<Person>,
    object: T,
    #[builder(default, setter(strip_option))]
    to: Option<SingleOrMany<UrlId<Person>>>,
}

pub type CreateNote = Create<Note>;

#[cfg(test)]
mod tests {
    use super::*;
    use apub_shared::model::resource_url::ResourceUrl;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize_create_note() {
        let post_note = r#"
            {
                "@context": "https://www.w3.org/ns/activitystreams",
                "summary": "Sally created a note",
                "id": "https://example.com/n/123456",
                "type": "Create",
                "actor": "https://example.com/u/bob",
                "object": {
                    "type": "Note",
                    "name": "A Simple Note",
                    "content": "This is a simple note"
                }
            }
        "#;

        let deserialized = serde_json::from_str::<CreateNote>(post_note).unwrap();

        let note = Note::builder()
            .content("This is a simple note".to_string())
            .build();
        let expected = CreateNote::builder()
            .context(
                "https://www.w3.org/ns/activitystreams"
                    .parse::<ResourceUrl>()
                    .unwrap()
                    .into(),
            )
            .id("https://example.com/n/123456".parse().unwrap())
            .actor("https://example.com/u/bob".parse().unwrap())
            .object(note)
            .build();

        assert_eq!(expected, deserialized)
    }
}
