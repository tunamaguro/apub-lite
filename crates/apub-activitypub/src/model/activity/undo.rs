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

use super::FollowPerson;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum UndoKind {
    #[default]
    Undo,
}

/// Undo activity
///
/// See
/// - https://www.w3.org/TR/activitypub/#undo-activity-outbox
/// - https://www.w3.org/TR/activitystreams-vocabulary/#dfn-undo
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Undo<Act, Obj> {
    #[serde(rename = "@context")]
    context: Context,
    id: UrlId<Undo<Act, Obj>>,
    #[serde(rename = "type")]
    #[builder(default)]
    kind: UndoKind,
    actor: UrlId<Act>,
    pub object: Obj,
}

impl<Act, Obj> Object for Undo<Act, Obj> {
    type Kind = UndoKind;
}

impl<Act, Obj> Activity for Undo<Act, Obj>
where
    Act: Actor,
    Obj: Actor,
{
    type ActorType = Act;
    type ObjectType = Obj;
    type TargetType = EmptyObject;
}

/// `Person`が何かからの`Follow`を`Undo`する
pub type UndoPersonFollow<Act> = Undo<Person, FollowPerson<Act>>;

#[cfg(test)]
mod tests {
    use crate::model::person::PersonUrl;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize_undo() {
        let undo = r#"
        {
            "@context":"https://www.w3.org/ns/activitystreams",
            "id":"https://activitypub.academy/users/dabena_durvaelol#follows/4243/undo",
            "type":"Undo",
            "actor":"https://activitypub.academy/users/dabena_durvaelol",
            "object":{
                "id":"https://activitypub.academy/6984660a-f515-4c51-bfc5-eb440db395b0",
                "type":"Follow",
                "actor":"https://activitypub.academy/users/dabena_durvaelol",
                "object":"https://5998e0c614b8ee9dd15fa09a84508ea1.serveo.net/users/alice"
            }
        }
    "#;

        let deserialized = serde_json::from_str::<UndoPersonFollow<Person>>(undo).unwrap();

        let actor_url = "https://activitypub.academy/users/dabena_durvaelol"
            .parse::<PersonUrl>()
            .unwrap();
        let follow = FollowPerson::builder()
            .context(None)
            .actor(actor_url.clone())
            .id(
                "https://activitypub.academy/6984660a-f515-4c51-bfc5-eb440db395b0"
                    .parse::<_>()
                    .unwrap(),
            )
            .object(
                "https://5998e0c614b8ee9dd15fa09a84508ea1.serveo.net/users/alice"
                    .parse::<_>()
                    .unwrap(),
            )
            .build();

        let expected = UndoPersonFollow::<Person>::builder()
            .context(Context::activity_context_url().clone().into())
            .actor(actor_url)
            .id(
                "https://activitypub.academy/users/dabena_durvaelol#follows/4243/undo"
                    .parse::<_>()
                    .unwrap(),
            )
            .object(follow)
            .build();

        assert_eq!(expected, deserialized);
    }
}
