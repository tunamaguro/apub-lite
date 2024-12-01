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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum FollowKind {
    #[default]
    Follow,
}

/// Follow activity
///
/// See https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Follow<Act, Obj> {
    #[serde(rename = "@context")]
    context: Option<Context>,
    id: UrlId<Follow<Act, Obj>>,
    #[serde(rename = "type")]
    #[builder(default)]
    kind: FollowKind,
    /// フォローを実行する`Actor`(e.g. `Person`, `Service`)
    pub actor: UrlId<Act>,
    /// フォローされる`Object`(e.g. `Person`, `Service`)
    pub object: UrlId<Obj>,
}

impl<Act, Obj> Object for Follow<Act, Obj> {
    type Kind = FollowKind;
}

impl<Act, Obj> Activity for Follow<Act, Obj>
where
    Act: Actor,
    Obj: Object,
{
    type ActorType = Act;
    type ObjectType = Obj;
    type TargetType = EmptyObject;
}

/// `Act`からの`Person`に対する`Follow`
pub type FollowPerson<Act> = Follow<Act, Person>;

#[cfg(test)]
mod tests {
    use super::*;
    use apub_shared::model::resource_url::ResourceUrl;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize_follow() {
        // Copy from `activity.academy` log
        let follow = r#"
            {
                "@context": "https://www.w3.org/ns/activitystreams",
                "id": "https://example.com/4849e989-2267-406c-aa84-3a4f6b0e7aab",
                "type": "Follow",
                "actor": "https://example.com/users/dodatus_dranapat",
                "object": "https://example.com/users/bob"
            }
        "#;

        let deserialized = serde_json::from_str::<FollowPerson<Person>>(follow).unwrap();
        let expected = FollowPerson::<Person>::builder()
            .context(Some(Context::activity_context_url().clone().into()))
            .id("https://example.com/4849e989-2267-406c-aa84-3a4f6b0e7aab"
                .parse::<ResourceUrl>()
                .unwrap()
                .into())
            .actor(
                "https://example.com/users/dodatus_dranapat"
                    .parse::<UrlId<Person>>()
                    .unwrap(),
            )
            .object(
                "https://example.com/users/bob"
                    .parse::<UrlId<Person>>()
                    .unwrap(),
            )
            .build();
        assert_eq!(expected, deserialized)
    }
}
