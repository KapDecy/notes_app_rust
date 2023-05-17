use crate::schema::notes;
use diesel::Insertable;
use serde::Serialize;
use uuid::Uuid;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=notes)]
pub struct NewNote {
    pub owner: Uuid,
    pub label: String,
    pub content: String,
}
