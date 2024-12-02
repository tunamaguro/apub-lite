use apub_shared::model::{id::UrlId, resource_url::ResourceUrl};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use crate::core::object::Object;

use super::context::Context;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CollectionKind {
    #[default]
    Collection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum OrderedCollectionKind {
    #[default]
    OrderedCollection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CollectionPageKind {
    #[default]
    CollectionPage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum OrderedCollectionPageKind {
    #[default]
    OrderedCollectionPage,
}

/// Activity Collection
///
/// See
/// - https://www.w3.org/TR/activitystreams-core/#collection
/// - https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collection
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(setter(strip_option, into)))]
pub struct Collection<T> {
    #[serde(rename = "@context")]
    context: Option<Context>,
    id: Option<UrlId<Collection<T>>>,
    #[serde(rename = "type")]
    #[builder(default,setter(!strip_option))]
    kind: CollectionKind,
    #[serde(flatten)]
    #[builder(setter(!strip_option))]
    base: CollectionBase<T>,
}

impl<T> Object for Collection<T> {
    type Kind = CollectionKind;
}

impl<T> Collection<T> {
    pub fn id(&self) -> Option<&UrlId<Self>> {
        self.id.as_ref()
    }

    pub fn items(&self) -> Option<&Vec<T>> {
        self.base.items.as_ref()
    }

    pub fn total(&self) -> Option<usize> {
        self.base.total_items
    }

    pub fn first(&self) -> Option<&ResourceUrl> {
        self.base.first.as_ref()
    }
}

/// Activity OrderedCollection
///
/// See
/// - https://www.w3.org/TR/activitystreams-core/#collection
/// - https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollection
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(setter(strip_option, into)))]
pub struct OrderedCollection<T> {
    #[serde(rename = "@context")]
    context: Option<Context>,
    id: Option<UrlId<OrderedCollection<T>>>,
    #[serde(rename = "type")]
    #[builder(default,setter(!strip_option))]
    kind: OrderedCollectionKind,
    #[serde(flatten)]
    #[builder(setter(!strip_option))]
    base: OrderedCollectionBase<T>,
}

impl<T> Object for OrderedCollection<T> {
    type Kind = OrderedCollectionKind;
}

impl<T> OrderedCollection<T> {
    pub fn id(&self) -> Option<&UrlId<Self>> {
        self.id.as_ref()
    }

    pub fn items(&self) -> Option<&Vec<T>> {
        self.base.ordered_items.as_ref()
    }

    pub fn total(&self) -> Option<usize> {
        self.base.total_items
    }

    pub fn first(&self) -> Option<&ResourceUrl> {
        self.base.first.as_ref()
    }
}

/// `Collection` or `OrderedCollection`
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum AnyCollection<T> {
    Collection(Collection<T>),
    OrderedCollection(OrderedCollection<T>),
}

impl<T> AnyCollection<T> {
    pub fn id(&self) -> Option<&ResourceUrl> {
        match self {
            AnyCollection::Collection(c) => c.id().map(|v| v.as_ref()),
            AnyCollection::OrderedCollection(c) => c.id().map(|v| v.as_ref()),
        }
    }

    pub fn items(&self) -> Option<&Vec<T>> {
        match self {
            AnyCollection::Collection(c) => c.items(),
            AnyCollection::OrderedCollection(c) => c.items(),
        }
    }

    pub fn total(&self) -> Option<usize> {
        match self {
            AnyCollection::Collection(c) => c.total(),
            AnyCollection::OrderedCollection(c) => c.total(),
        }
    }

    pub fn first(&self) -> Option<&ResourceUrl> {
        match self {
            AnyCollection::Collection(c) => c.first(),
            AnyCollection::OrderedCollection(c) => c.first(),
        }
    }
}

/// Activity CollectionPage
///
/// See
/// - https://www.w3.org/TR/activitystreams-core/#paging
/// - https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collectionpage
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(setter(strip_option)))]
pub struct CollectionPage<T> {
    #[serde(rename = "@context")]
    context: Option<Context>,
    id: Option<UrlId<CollectionPage<T>>>,
    #[serde(rename = "type")]
    #[builder(default,setter(!strip_option))]
    kind: CollectionPageKind,
    #[serde(flatten)]
    #[builder(setter(!strip_option))]
    base: CollectionBase<T>,
    #[serde(flatten)]
    #[builder(setter(!strip_option))]
    page: CollectionPageBase,
}

impl<T> Object for CollectionPage<T> {
    type Kind = CollectionPageKind;
}

impl<T> CollectionPage<T> {
    pub fn id(&self) -> Option<&UrlId<Self>> {
        self.id.as_ref()
    }

    pub fn items(&self) -> Option<&Vec<T>> {
        self.base.items.as_ref()
    }

    pub fn total(&self) -> Option<usize> {
        self.base.total_items
    }

    pub fn next(&self) -> Option<&ResourceUrl> {
        self.page.next.as_ref()
    }

    pub fn prev(&self) -> Option<&ResourceUrl> {
        self.page.prev.as_ref()
    }
}

/// Activity OrderedCollectionPage
///
/// See
/// - https://www.w3.org/TR/activitystreams-core/#paging
/// - https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollectionpage
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(setter(strip_option)))]
pub struct OrderedCollectionPage<T> {
    #[serde(rename = "@context")]
    context: Option<Context>,
    id: Option<UrlId<OrderedCollectionPage<T>>>,
    #[serde(rename = "type")]
    #[builder(default,setter(!strip_option))]
    kind: OrderedCollectionPageKind,
    #[serde(flatten)]
    #[builder(setter(!strip_option))]
    base: OrderedCollectionBase<T>,
    #[serde(flatten)]
    #[builder(setter(!strip_option))]
    page: CollectionPageBase,
}

impl<T> Object for OrderedCollectionPage<T> {
    type Kind = OrderedCollectionPageKind;
}

impl<T> OrderedCollectionPage<T> {
    pub fn id(&self) -> Option<&UrlId<Self>> {
        self.id.as_ref()
    }

    pub fn items(&self) -> Option<&Vec<T>> {
        self.base.ordered_items.as_ref()
    }

    pub fn total(&self) -> Option<usize> {
        self.base.total_items
    }

    pub fn next(&self) -> Option<&ResourceUrl> {
        self.page.next.as_ref()
    }

    pub fn prev(&self) -> Option<&ResourceUrl> {
        self.page.prev.as_ref()
    }
}

/// `CollectionPage` or `OrderedCollectionPage`
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum AnyCollectionPage<T> {
    CollectionPage(CollectionPage<T>),
    OrderedCollectionPage(OrderedCollectionPage<T>),
}

impl<T> AnyCollectionPage<T> {
    pub fn id(&self) -> Option<&ResourceUrl> {
        match self {
            AnyCollectionPage::CollectionPage(p) => p.id().map(|v| v.as_ref()),
            AnyCollectionPage::OrderedCollectionPage(p) => p.id().map(|v| v.as_ref()),
        }
    }

    pub fn items(&self) -> Option<&Vec<T>> {
        match self {
            AnyCollectionPage::CollectionPage(p) => p.items(),
            AnyCollectionPage::OrderedCollectionPage(p) => p.items(),
        }
    }

    pub fn total(&self) -> Option<usize> {
        match self {
            AnyCollectionPage::CollectionPage(p) => p.total(),
            AnyCollectionPage::OrderedCollectionPage(p) => p.total(),
        }
    }

    pub fn next(&self) -> Option<&ResourceUrl> {
        match self {
            AnyCollectionPage::CollectionPage(p) => p.next(),
            AnyCollectionPage::OrderedCollectionPage(p) => p.next(),
        }
    }

    pub fn prev(&self) -> Option<&ResourceUrl> {
        match self {
            AnyCollectionPage::CollectionPage(p) => p.prev(),
            AnyCollectionPage::OrderedCollectionPage(p) => p.prev(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct CollectionBase<T> {
    total_items: Option<usize>,
    items: Option<Vec<T>>,
    first: Option<ResourceUrl>,
    last: Option<ResourceUrl>,
}

impl<T> Default for CollectionBase<T> {
    fn default() -> Self {
        Self::builder().total_items(0).items(Vec::new()).build()
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct OrderedCollectionBase<T> {
    total_items: Option<usize>,
    ordered_items: Option<Vec<T>>,
    first: Option<ResourceUrl>,
    last: Option<ResourceUrl>,
}

impl<T> Default for OrderedCollectionBase<T> {
    fn default() -> Self {
        Self::builder()
            .total_items(0)
            .ordered_items(Vec::new())
            .build()
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct CollectionPageBase {
    next: Option<ResourceUrl>,
    prev: Option<ResourceUrl>,
}

#[cfg(test)]
mod tests {
    use crate::model::note::Note;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize_collection() {
        let collection = r#"
            {
                "@context": "https://www.w3.org/ns/activitystreams",
                "id": "https://example.com/users/alice/followers",
                "type": "Collection",
                "totalItems": 2,
                "items": [
                    "https://example.com/users/bob",
                    "https://example.com/users/charlie"
                ]
            }
        "#;

        let deserialized = serde_json::from_str(collection).unwrap();

        let collection_base = CollectionBase::<_>::builder()
            .items(
                [
                    "https://example.com/users/bob",
                    "https://example.com/users/charlie",
                ]
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect(),
            )
            .total_items(2)
            .build();
        let expected = Collection::<ResourceUrl>::builder()
            .base(collection_base)
            .context(Context::default())
            .id("https://example.com/users/alice/followers"
                .parse::<UrlId<_>>()
                .unwrap())
            .build();

        assert_eq!(expected, deserialized)
    }

    #[test]
    fn test_serialize_ordered_collection() {
        let collection = r#"
            {
                "@context": "https://www.w3.org/ns/activitystreams",
                "id": "https://example.com/users/alice/outbox",
                "type": "OrderedCollection",
                "totalItems": 1,
                "orderedItems": [
                    {
                        "id": "https://example.com/users/alice/statuses/1",
                        "type": "Note",
                        "content": "Hello World!"
                    }
                ]
            }
        "#;

        let deserialized = serde_json::from_str(collection).unwrap();

        let collection_base = OrderedCollectionBase::<_>::builder()
            .ordered_items(vec![Note::builder()
                .id("https://example.com/users/alice/statuses/1"
                    .parse()
                    .unwrap())
                .content("Hello World!".into())
                .build()])
            .total_items(1)
            .build();
        let expected = OrderedCollection::<Note>::builder()
            .base(collection_base)
            .context(Context::default())
            .id("https://example.com/users/alice/outbox"
                .parse::<UrlId<_>>()
                .unwrap())
            .build();

        assert_eq!(expected, deserialized)
    }

    #[test]
    fn test_serialize_ordered_collection_page() {
        // sampled from https://mastodon.social/users/Mastodon/collections/followers
        let collection = r#"
            {
                "@context": "https://www.w3.org/ns/activitystreams",
                "id": "https://example.com/users/User1/followers?page=1",
                "type": "OrderedCollectionPage",
                "totalItems": 822847,
                "next": "https://example.com/users/User1/followers?page=2",
                "partOf": "https://example.com/users/User1/followers",
                "orderedItems": [
                    "https://example.com/users/UserA",
                    "https://example.com/users/UserB",
                    "https://example.com/users/UserC",
                    "https://example.com/users/UserD",
                    "https://example.com/users/UserE",
                    "https://example.com/users/UserF",
                    "https://example.com/users/UserG",
                    "https://example.com/users/UserH",
                    "https://example.com/users/UserI",
                    "https://example.com/users/UserJ",
                    "https://example.com/users/UserK",
                    "https://example.com/users/UserL"
                ]
            }
        "#;

        let deserialized = serde_json::from_str(collection).unwrap();

        let collection_base = OrderedCollectionBase::<_>::builder()
            .ordered_items(
                [
                    "https://example.com/users/UserA",
                    "https://example.com/users/UserB",
                    "https://example.com/users/UserC",
                    "https://example.com/users/UserD",
                    "https://example.com/users/UserE",
                    "https://example.com/users/UserF",
                    "https://example.com/users/UserG",
                    "https://example.com/users/UserH",
                    "https://example.com/users/UserI",
                    "https://example.com/users/UserJ",
                    "https://example.com/users/UserK",
                    "https://example.com/users/UserL",
                ]
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect(),
            )
            .total_items(822847)
            .build();

        let page = CollectionPageBase::builder()
            .next(
                "https://example.com/users/User1/followers?page=2"
                    .parse()
                    .unwrap(),
            )
            .build();

        let expected = OrderedCollectionPage::<ResourceUrl>::builder()
            .base(collection_base)
            .context(Context::default())
            .id("https://example.com/users/User1/followers?page=1"
                .parse::<UrlId<_>>()
                .unwrap())
            .page(page)
            .build();

        assert_eq!(expected, deserialized)
    }
}
