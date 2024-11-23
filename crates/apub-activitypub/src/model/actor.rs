use apub_shared::model::resource_url::ResourceUrl;

pub trait Actor {
    fn id(&self) -> &ResourceUrl;
    fn inbox(&self) -> &ResourceUrl;
}
