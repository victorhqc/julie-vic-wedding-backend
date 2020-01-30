use super::schema::{confirmed_users, tables, users};
use crate::attend_status_type::AttendStatus;
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
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

#[derive(Debug, Serialize, Queryable, Associations, PartialEq)]
#[table_name = "confirmed_users"]
#[belongs_to(User)]
#[belongs_to(Table)]
pub struct ConfirmedUser {
    pub user_id: Uuid,
    pub will_attend: AttendStatus,
    pub table_id: Option<Uuid>,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "confirmed_users"]
pub struct NewConfirmedUser {
    pub user_id: Uuid,
    pub will_attend: AttendStatus,
    pub table_id: Option<Uuid>,
}
