use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::*;
use std::io::Write;

#[derive(SqlType, QueryId)]
#[postgres(type_name = "available_language")]
pub struct AvailableLanguageType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, QueryId, Serialize, Deserialize)]
#[sql_type = "AvailableLanguageType"]
pub enum AvailableLanguage {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
}

impl ToSql<AvailableLanguageType, Pg> for AvailableLanguage {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            AvailableLanguage::En => out.write_all(b"en")?,
            AvailableLanguage::Es => out.write_all(b"es")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<AvailableLanguageType, Pg> for AvailableLanguage {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"en" => Ok(AvailableLanguage::En),
            b"es" => Ok(AvailableLanguage::Es),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
