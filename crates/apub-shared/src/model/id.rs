use std::{marker::PhantomData, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::model::resource_uri::ResourceUri;

use super::resource_uri::ResourceUriError;

/// リソースにつけられた`Uri`
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct UriId<T> {
    resource_uri: ResourceUri,
    _marker: PhantomData<T>,
}

impl<T> Deref for UriId<T> {
    type Target = ResourceUri;

    fn deref(&self) -> &Self::Target {
        &self.resource_uri
    }
}

impl<T> From<ResourceUri> for UriId<T> {
    fn from(uri: ResourceUri) -> Self {
        Self {
            resource_uri: uri,
            _marker: PhantomData,
        }
    }
}

impl<T> std::fmt::Display for UriId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.resource_uri)
    }
}

impl<T> FromStr for UriId<T> {
    type Err = ResourceUriError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = s.parse()?;
        Ok(Self {
            resource_uri: uri,
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
