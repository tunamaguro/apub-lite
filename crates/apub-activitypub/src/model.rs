use serde::{Deserialize, Serialize};

pub mod acct_uri;
pub mod activity;
pub mod actor;
pub mod context;
pub mod key;
pub mod note;
pub mod person;
pub mod webfinger;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOrMany<T> {
    Many(Vec<T>),
    Single(T),
}

impl<T> From<T> for SingleOrMany<T> {
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T> From<Vec<T>> for SingleOrMany<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Many(value)
    }
}
