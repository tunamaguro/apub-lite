use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::model::resource_uri::ResourceUri;

/// リソースにつけられた`Uri`
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct UriId<T> {
    resource_uri: ResourceUri,
    _marker: PhantomData<T>,
}

impl<T> From<ResourceUri> for UriId<T> {
    fn from(uri: ResourceUri) -> Self {
        Self {
            resource_uri: uri,
            _marker: PhantomData,
        }
    }
}
