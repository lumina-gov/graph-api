//! Implements utility type for JSON, JSONB field handling in diesel
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::{sql_types};
use diesel::{deserialize::FromSql, serialize::ToSql};
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use std::io::Write;
use std::ops::{Deref, DerefMut};

/// Wrapper type that implements Json data handling for postgres diesel connection
///
/// Use as wrapper for fields that are stored in Jsonb format
/// ```ignore
/// #[derive(serde::Serialize, serde::Deserialize, Debug)]
/// pub struct ComplexStruct {
///   // ...
/// }
///
/// #[derive(serde::Serialize, serde::Deserialize,
///          diesel::Queryable, diesel::Insertable)]
/// pub struct ExampleTable {
///     // Field that will be stored in Json, Jsonb format
///     pub jsonb_field: jsonB<ComplexStruct>,
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, AsExpression)]
#[serde(transparent)]
#[sql_type = "sql_types::Jsonb"]
pub struct JsonB<T: Sized>(pub T);

impl<T> JsonB<T> {
    pub fn new(value: T) -> JsonB<T> {
        Self(value)
    }
}

impl<T> Deref for JsonB<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for JsonB<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for JsonB<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for JsonB<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> FromSql<sql_types::Jsonb, Pg> for JsonB<T>
where
    T: std::fmt::Debug + DeserializeOwned,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<sql_types::Jsonb, Pg>>::from_sql(bytes)?;
        Ok(JsonB(serde_json::from_value::<T>(value)?))
    }
}



impl<T> ToSql<sql_types::Jsonb, Pg> for JsonB<T>
where
    T: std::fmt::Debug + Serialize,
{
    fn to_sql<'b>(
        &self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}

impl<T> PartialEq for JsonB<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: DeserializeOwned> FromSqlRow<sql_types::Jsonb, Pg> for JsonB<T> {
    fn build_from_row<'a>(row: &impl diesel::row::Row<'a, Pg>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSqlRow<sql_types::Jsonb, Pg>>::build_from_row(row)?;
        Ok(JsonB(serde_json::from_value::<T>(value)?))
    }
}

// impl<T: DeserializeOwned> AsExpression<sql_types::Jsonb> for JsonB<T> {


//     fn as_expression(self) -> Self::Expression {
//         diesel::expression::bound::Bound::new(self, sql_types::Jsonb)
//     }
// }