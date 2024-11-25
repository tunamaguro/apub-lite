use super::{actor::Actor, object::Object};

/// 自身がActivity Stream2.0の`Activity`を継承していることを表明する
///
/// See https://www.w3.org/TR/activitystreams-core/#activities
pub trait Activity: Object {
    type ActorType: Actor;
    type ObjectType: Object;
    type TargetType: Object;
    /// このアクティビティを実行する`Actor`を返せれば返す
    fn actor(&self) -> Option<&Self::ActorType> {
        None
    }

    /// このアクティビティの対象となる`Object`を返せれば返す
    fn activity_object(&self) -> Option<&Self::ObjectType> {
        None
    }
    /// このアクティビティのターゲットとなる`Object`を返せれば返す
    ///
    /// See https://www.w3.org/TR/activitystreams-vocabulary/#dfn-target
    fn activity_target(&self) -> Option<&Self::TargetType> {
        None
    }
}
