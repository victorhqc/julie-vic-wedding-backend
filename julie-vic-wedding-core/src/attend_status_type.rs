use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::*;
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "attend_status")]
pub struct AttendStatusType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize)]
#[sql_type = "AttendStatusType"]
pub enum AttendStatus {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "yes+one")]
    YesPlusOne,
}

impl AttendStatus {
    pub fn parse(&self) -> (bool, bool) {
        let (will_attend, plus_one) = match self {
            AttendStatus::No => (false, false),
            AttendStatus::Yes => (true, false),
            AttendStatus::YesPlusOne => (true, true),
        };

        (will_attend, plus_one)
    }
}

impl ToSql<AttendStatusType, Pg> for AttendStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            AttendStatus::No => out.write_all(b"no")?,
            AttendStatus::Yes => out.write_all(b"yes")?,
            AttendStatus::YesPlusOne => out.write_all(b"yes+one")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<AttendStatusType, Pg> for AttendStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"no" => Ok(AttendStatus::No),
            b"yes" => Ok(AttendStatus::Yes),
            b"yes+one" => Ok(AttendStatus::YesPlusOne),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// impl HasSqlType for AttendStatus {
//
// }
