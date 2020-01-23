use super::schema::{confirmed_users, tables, users};
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable)]
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
pub struct NewUser<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub last_name: Option<&'a str>,
    pub email: &'a str,
}

#[derive(Debug, Serialize, Queryable)]
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

#[derive(Debug, Serialize, Queryable)]
pub struct ConfirmedUser {
    pub user_id: Uuid,
    pub table_id: Uuid,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "confirmed_users"]
pub struct NewConfirmedUser<'a> {
    pub user_id: &'a Uuid,
    pub table_id: &'a Uuid,
}
