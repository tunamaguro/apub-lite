use std::{marker::PhantomData, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::model::resource_url::ResourceUrl;

use super::resource_url::ResourceUrlError;

/// ある特定のリソースを示す`Url``
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct UrlId<T> {
    resource_url: ResourceUrl,
    _marker: PhantomData<T>,
}

impl<T> Deref for UrlId<T> {
    type Target = ResourceUrl;

    fn deref(&self) -> &Self::Target {
        &self.resource_url
    }
}

impl<T> From<ResourceUrl> for UrlId<T> {
    fn from(uri: ResourceUrl) -> Self {
        Self {
            resource_url: uri,
            _marker: PhantomData,
        }
    }
}

impl<T> From<UrlId<T>> for ResourceUrl {
    fn from(value: UrlId<T>) -> Self {
        value.resource_url
    }
}

impl<T> std::fmt::Display for UrlId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.resource_url)
    }
}

impl<T> FromStr for UrlId<T> {
    type Err = ResourceUrlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = s.parse()?;
        Ok(Self {
            resource_url: uri,
            _marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Id<T> {
    uuid: uuid::Uuid,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        let uuid = uuid::Uuid::now_v7();
        Self {
            uuid,
            _marker: PhantomData,
        }
    }
}

impl<T> AsRef<uuid::Uuid> for Id<T> {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.uuid
    }
}

impl<T> Deref for Id<T> {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.uuid
    }
}

impl<T> From<uuid::Uuid> for Id<T> {
    fn from(value: uuid::Uuid) -> Self {
        Self {
            uuid: value,
            _marker: PhantomData,
        }
    }
}

impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

impl<T> FromStr for Id<T> {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = uuid::Uuid::from_str(s)?;
        Ok(Self {
            uuid: id,
            ..Default::default()
        })
    }
}
