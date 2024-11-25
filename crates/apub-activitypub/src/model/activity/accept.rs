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
    model::{context::Context, person::Person},
};

use super::follow::FollowPerson;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum AcceptKind {
    #[default]
    Accept,
}

/// Accept activity
///
/// See
/// - https://www.w3.org/TR/activitypub/#follow-activity-outbox
/// - https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Accept<Act, Obj> {
    #[serde(rename = "@context")]
    context: Context,
    id: UrlId<Accept<Act, Obj>>,
    #[serde(rename = "type")]
    #[builder(default)]
    kind: AcceptKind,
    actor: UrlId<Act>,
    object: Obj,
}

impl<Act, Obj> Object for Accept<Act, Obj> {
    type Kind = AcceptKind;
}

impl<Act, Obj> Activity for Accept<Act, Obj>
where
    Act: Actor,
    Obj: Actor,
{
    type ActorType = Act;
    type ObjectType = Obj;
    type TargetType = EmptyObject;
}

/// `Person`が何かからの`Follow`を`Accept`する
pub type AcceptPersonFollow<Act> = Accept<Person, FollowPerson<Act>>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize_accept() {
        let accept = r#"
            {
            "@context": "https://www.w3.org/ns/activitystreams",
            "id": "https://example.com/activities/12345",
            "type": "Accept",
            "actor": "https://example.com/users/bob",
            "object": {
                "@context": "https://www.w3.org/ns/activitystreams",
                "id": "https://example.com/activities/67890",
                "type": "Follow",
                "actor": "https://example.com/users/alice",
                "object": "https://example.com/users/bob"
                }
            }
        "#;
        let deserialized = serde_json::from_str::<AcceptPersonFollow<Person>>(accept).unwrap();

        let follow = FollowPerson::<Person>::builder()
            .context(Context::activity_context_url().clone().into())
            .id("https://example.com/activities/67890"
                .parse::<UrlId<_>>()
                .unwrap())
            .actor(
                "https://example.com/users/alice"
                    .parse::<UrlId<_>>()
                    .unwrap(),
            )
            .object("https://example.com/users/bob".parse::<UrlId<_>>().unwrap())
            .build();

        let expected = AcceptPersonFollow::<Person>::builder()
            .context(Context::activity_context_url().clone().into())
            .id("https://example.com/activities/12345"
                .parse::<UrlId<_>>()
                .unwrap())
            .actor("https://example.com/users/bob".parse::<UrlId<_>>().unwrap())
            .object(follow)
            .build();

        assert_eq!(expected, deserialized)
    }
}
