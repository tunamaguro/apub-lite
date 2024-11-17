use apub_kernel::model::user::User;
use sqlx::types::Uuid;

pub struct UserRow {
    pub id: Uuid,
    pub name: String,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        let UserRow { name, id } = value;
        User::builder().name(name).id(id.into()).build()
    }
}
