use serde::{Deserialize, Serialize};

pub mod acct_uri;
pub mod actor;
pub mod context;
pub mod key;
pub mod note;
pub mod person;
pub mod webfinger;
pub mod activity;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
enum SingleOrVec<T> {
    Single(T),
    Vec(Vec<T>),
}

impl<T> SingleOrVec<T> {
    fn as_single(&self) -> Option<&T> {
        match self {
            SingleOrVec::Single(v) => Some(v),
            _ => None,
        }
    }

    fn as_vec(&self) -> Option<&Vec<T>> {
        match self {
            SingleOrVec::Vec(v) => Some(v),
            _ => None,
        }
    }
}

impl<T> From<T> for SingleOrVec<T> {
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T> From<Vec<T>> for SingleOrVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Vec(value)
    }
}
