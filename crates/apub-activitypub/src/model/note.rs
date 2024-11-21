use apub_shared::model::id::UriId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use super::{context::Context, SingleOrVec};

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
    id: Option<UriId<Note>>,
    #[serde(rename = "type")]
    #[builder(setter(!strip_option))]
    kind: NoteKind,
    #[builder(setter(!strip_option))]
    content: String,
    published: Option<String>,
    to: Option<SingleOrVec<String>>,
    in_reply_to: Option<UriId<Note>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use apub_shared::model::resource_uri::ResourceUri;
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
                    .parse::<ResourceUri>()
                    .unwrap()
                    .into(),
            )
            .build();

        assert_eq!(expected, deserialized)
    }
}
