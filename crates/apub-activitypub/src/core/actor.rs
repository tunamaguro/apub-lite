use super::object::Object;

/// 自身がActivity Stream2.0の`Actor`を継承していることを表明する
///
/// See https://www.w3.org/TR/activitystreams-core/#actors
pub trait Actor: Object {}
