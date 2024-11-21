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
pub enum SingleOrVec<T> {
    Single(T),
    Vec(Vec<T>),
}

impl<T> SingleOrVec<T> {
    #[allow(dead_code)]
    fn as_single(&self) -> Option<&T> {
        match self {
            SingleOrVec::Single(v) => Some(v),
            _ => None,
        }
    }
    #[allow(dead_code)]
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
