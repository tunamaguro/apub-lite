use serde::{de::DeserializeOwned, Serialize};

/// 自身がActivity Stream2.0の`Object`を継承していることを表明する
///
/// See https://www.w3.org/TR/activitystreams-core/#object
pub trait Object: DeserializeOwned + Serialize {
    type Kind: DeserializeOwned + Serialize + Default;
    /// このオブジェクトの`type`を返す
    fn kind() -> Self::Kind {
        Self::Kind::default()
    }
}


