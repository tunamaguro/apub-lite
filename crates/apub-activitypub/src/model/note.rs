use std::sync::LazyLock;

use apub_shared::model::{id::UrlId, resource_url::ResourceUrl};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use super::{context::Context, SingleOrMany};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum NoteKind {
    #[default]
    Note,
}

/// Activity Note Object
///
/// See https://www.w3.org/ns/activitystreams#
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Note {
    #[serde(rename = "@context")]
    context: Option<Context>,
    id: Option<UrlId<Note>>,
    #[serde(rename = "type")]
    #[builder(setter(!strip_option))]
    kind: NoteKind,
    #[builder(setter(!strip_option))]
    content: String,
    published: Option<String>,
    to: Option<SingleOrMany<ResourceUrl>>,
    in_reply_to: Option<UrlId<Note>>,
}

impl Note {
    /// Return public address  
    ///
    /// See https://w3c.github.io/activitypub/#public-addressing
    pub fn public_address() -> &'static ResourceUrl {
        static PUBLIC: LazyLock<ResourceUrl> = LazyLock::new(|| {
            "https://www.w3.org/ns/activitystreams#Public"
                .parse::<ResourceUrl>()
                .unwrap()
        });
        &PUBLIC
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apub_shared::model::resource_url::ResourceUrl;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize_note_spec() {
        // https://www.w3.org/TR/activitystreams-vocabulary/#dfn-note
        let note_json = r#"
            {
            "@context": "https://www.w3.org/ns/activitystreams",
            "type": "Note",
            "name": "A Word of Warning",
            "content": "Looks like it is going to rain today. Bring an umbrella!"
            }
        "#;

        let deserialized = serde_json::from_str::<Note>(note_json).unwrap();

        let expected = Note::builder()
            .content("Looks like it is going to rain today. Bring an umbrella!".to_string())
            .context(
                "https://www.w3.org/ns/activitystreams"
                    .parse::<ResourceUrl>()
                    .unwrap()
                    .into(),
            )
            .build();

        assert_eq!(expected, deserialized)
    }
}
