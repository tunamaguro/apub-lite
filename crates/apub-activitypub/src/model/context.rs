use std::{collections::HashMap, sync::LazyLock};

use apub_shared::model::resource_url::ResourceUrl;
use serde::{Deserialize, Serialize};

use crate::shared::SingleOrMany;

/// ActivityPub Context
///
/// See https://www.w3.org/TR/activitystreams-core/#jsonld
pub type Context = SingleOrMany<ContextInner>;

impl Context {
    pub fn activity_context_url() -> &'static ContextInner {
        static ACTIVITY_CONTEXT: LazyLock<ContextInner> = LazyLock::new(|| {
            let context_url = "https://www.w3.org/ns/activitystreams"
                .parse::<ResourceUrl>()
                .unwrap();
            ContextInner::Uri(context_url)
        });
        &ACTIVITY_CONTEXT
    }
}

impl From<ResourceUrl> for Context {
    fn from(value: ResourceUrl) -> Self {
        Self::Single(value.into())
    }
}

impl From<Vec<ResourceUrl>> for Context {
    fn from(value: Vec<ResourceUrl>) -> Self {
        let arr = value
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<ContextInner>>();
        arr.into()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextInner {
    Uri(ResourceUrl),
    Object(HashMap<String, serde_json::Value>),
    Unknown(serde_json::Value),
}

impl From<ResourceUrl> for ContextInner {
    fn from(value: ResourceUrl) -> Self {
        Self::Uri(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestContext {
        #[serde(rename = "@context")]
        context: Context,
    }

    #[test]
    fn context_with_string() {
        // https://www.w3.org/TR/activitystreams-core/#h-example-1context
        let v = r#"
        {
        "@context": "https://www.w3.org/ns/activitystreams"
        }
        "#;

        let s: TestContext = serde_json::from_str(v).unwrap();

        println!("{:?}", s);
    }

    #[test]
    fn context_with_object() {
        // https://www.w3.org/TR/activitystreams-core/#h-example-2context
        let v = r#"
        {
            "@context": {
                "@vocab": "https://www.w3.org/ns/activitystreams",
                "ext": "https://canine-extension.example/terms/",
                "@language": "en"
            }
        }
        "#;

        let s: TestContext = serde_json::from_str(v).unwrap();

        println!("{:?}", s);
    }

    #[test]
    fn context_with_array() {
        let v = r#"
        {
            "@context": [
                "https://www.w3.org/ns/activitystreams",
                {
                    "css": "http://www.w3.org/ns/oa#styledBy"
                }
            ]
        }
        "#;

        let s: TestContext = serde_json::from_str(v).unwrap();

        println!("{:?}", s);
    }
}
