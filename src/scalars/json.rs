//! [JSON] wrapper/[newtype struct] used in many database documents.
//!
//! [JSON]: https://tools.ietf.org/html/rfc8259#section-3
//! [newtype struct]: https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html
//! [scalar]: https://graphql.org/learn/schema/#scalar-types

use std::{
    borrow::Borrow, collections::BTreeMap, convert::TryFrom, default::Default, hash::Hash,
    str::FromStr, string::String,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map as JsonMap, Value as JsonValue};

/// A [JSON] object (key => value map).
///
/// [JSON]: https://tools.ietf.org/html/rfc8259#section-3
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Map(JsonMap<String, JsonValue>);

impl Map {
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form must match the ordering on the key type.
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&JsonValue>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.0.get(key)
    }

    /// Returns true if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Map {
    #[inline]
    fn default() -> Self {
        Map(JsonMap::<String, JsonValue>::new())
    }
}

impl From<JsonValue> for Map {
    #[inline]
    fn from(value: JsonValue) -> Self {
        match value.as_object() {
            Some(val) => Map(val.to_owned()),
            None if value.is_null() => Self::default(),
            None => panic!("invalid JSON object: `{}`", &value),
        }
    }
}

impl From<BTreeMap<String, async_graphql::Value>> for Map {
    #[inline]
    fn from(value: BTreeMap<String, async_graphql::Value>) -> Self {
        let result = value
            .into_iter()
            .map(|(k, v)| (k, JsonValue::from(v)))
            .collect();

        Map(result)
    }
}

impl FromStr for Map {
    type Err = serde_json::error::Error;

    #[inline]
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(val)
    }
}

#[async_graphql::Scalar]
impl async_graphql::ScalarType for Map {
    #[inline]
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::Null => Ok(Map::default()),
            async_graphql::Value::String(val) => Ok(val.parse::<Map>()?),
            async_graphql::Value::Object(val) => Ok(Map::try_from(val)?),
            async_graphql::Value::Variable(_)
            | async_graphql::Value::Int(_)
            | async_graphql::Value::Float(_)
            | async_graphql::Value::Boolean(_)
            | async_graphql::Value::Enum(_)
            | async_graphql::Value::List(_)
            | async_graphql::Value::Upload(_) => {
                Err(async_graphql::InputValueError::ExpectedType(value))
            }
        }
    }

    #[inline]
    fn to_value(&self) -> async_graphql::Value {
        json!(&self.0).into()
    }
}
