use apub_shared::model::resource_uri::ResourceUri;

pub trait Actor {
    fn id(&self) -> &ResourceUri;
    fn inbox(&self) -> &ResourceUri;
}
