use super::schema::{confirmed_users, tables, tokens, users};
use crate::attend_status_type::AttendStatus;
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use serde::ser::{Serialize as SerdeSerialize, SerializeStruct, Serializer};
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub last_name: Option<String>,
    pub email: String,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub last_name: Option<String>,
    pub email: String,
}

impl NewUser {
    pub fn new(name: String, last_name: Option<String>, email: String) -> Self {
        NewUser {
            id: Uuid::new_v4(),
            name,
            last_name,
            email,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable)]
#[table_name = "tables"]
pub struct Table {
    pub id: Uuid,
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Insertable)]
#[table_name = "tables"]
pub struct NewTable<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub alias: Option<&'a str>,
}

#[derive(Debug, Queryable, Associations, PartialEq)]
#[table_name = "confirmed_users"]
#[belongs_to(User)]
#[belongs_to(Table)]
pub struct ConfirmedUser {
    pub user_id: Uuid,
    pub will_attend: AttendStatus,
    pub table_id: Option<Uuid>,
    pub token_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl SerdeSerialize for ConfirmedUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (will_attend, plus_one) = self.will_attend.parse();

        let mut state = serializer.serialize_struct("ConfirmedUser", 6)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("table_id", &self.table_id)?;
        state.serialize_field("will_attend", &will_attend)?;
        state.serialize_field("plus_one", &plus_one)?;
        state.serialize_field("created_at", &self.created_at.timestamp())?;
        state.serialize_field("updated_at", &self.updated_at.timestamp())?;
        state.serialize_field("token_id", &self.token_id)?;
        state.end()
    }
}

#[derive(Insertable)]
#[table_name = "confirmed_users"]
pub struct NewConfirmedUser {
    pub user_id: Uuid,
    pub will_attend: AttendStatus,
    pub table_id: Option<Uuid>,
    pub token_id: Uuid,
}

#[derive(Debug, Serialize, Queryable, Identifiable)]
#[table_name = "tokens"]
pub struct Token {
    pub id: Uuid,
    pub token: String,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tokens"]
pub struct NewToken {
    pub id: Uuid,
    token: String,
}
