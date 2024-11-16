use std::collections::HashMap;

use apub_shared::model::resource_uri::ResourceUri;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Context {
    Many(Vec<ContextInner>),
    Single(ContextInner),
}

impl From<ResourceUri> for Context {
    fn from(value: ResourceUri) -> Self {
        Self::Single(value.into())
    }
}

impl From<Vec<ResourceUri>> for Context {
    fn from(value: Vec<ResourceUri>) -> Self {
        let arr = value.into_iter().map(|v| v.into()).collect::<Vec<_>>();
        Self::Many(arr)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextInner {
    Uri(ResourceUri),
    Object(HashMap<String, serde_json::Value>),
    Unknown(serde_json::Value),
}

impl From<ResourceUri> for ContextInner {
    fn from(value: ResourceUri) -> Self {
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
