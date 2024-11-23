use apub_shared::model::id::UrlId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use crate::{
    core::{
        activity::Activity,
        actor::Actor,
        object::{EmptyObject, Object},
    },
    model::{context::Context, note::Note, person::Person},
    shared::SingleOrMany,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CreateKind {
    #[default]
    Create,
}

/// Create activity
///
/// See
/// - https://www.w3.org/TR/activitypub/#create-activity-outbox 
/// - https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Create<Act, Obj> {
    #[serde(rename = "@context")]
    context: Context,
    id: UrlId<Create<Act, Obj>>,
    #[serde(rename = "type")]
    #[builder(default)]
    kind: CreateKind,
    actor: UrlId<Act>,
    object: Obj,
    #[builder(default, setter(strip_option))]
    to: Option<SingleOrMany<UrlId<Act>>>,
}

impl<Act, Obj> Object for Create<Act, Obj> {
    type Kind = CreateKind;
}

impl<Act, Obj> Activity for Create<Act, Obj>
where
    Act: Actor,
    Obj: Object,
{
    type ActorType = Act;
    type ObjectType = Obj;
    type TargetType = EmptyObject;
}

/// `Person`が`Note`を作成する`Activity`
pub type CreatePersonNote = Create<Person, Note>;

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

        let deserialized = serde_json::from_str::<CreatePersonNote>(post_note).unwrap();

        let note = Note::builder()
            .content("This is a simple note".to_string())
            .build();
        let expected = CreatePersonNote::builder()
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
