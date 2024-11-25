use serde::{Deserialize, Serialize};

/// 自身がActivity Stream2.0の`Object`を継承していることを表明する
///
/// See https://www.w3.org/TR/activitystreams-core/#object
pub trait Object {
    type Kind: Default;
    /// このオブジェクトの`type`を返す
    fn kind() -> Self::Kind {
        Self::Kind::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ObjectKind {
    #[default]
    Object,
}

/// `type`以外を持たない`Object`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyObject {
    #[serde(rename = "type")]
    kind: ObjectKind,
}

impl Object for EmptyObject {
    type Kind = ObjectKind;
}
