use apub_kernel::user::model::User;
use sqlx::types::Uuid;

pub struct UserRow {
    pub user_id: Uuid,
    pub name: String,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        let UserRow { name, user_id } = value;
        User::builder().name(name).id(user_id.into()).build()
    }
}
