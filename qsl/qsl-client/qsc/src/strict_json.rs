//! Strict decoding of authenticated JSON records (NA-0788 F04/S1; C01 T1 rows 15 and 21,
//! C07 T6). A stored record is state, not an update: a repeated map key is contradictory and
//! refused, never resolved last-value-wins, and no field is filled in from its absence.
//! These helpers only parse; they never repair or rewrite input. Each decoder that uses them
//! keeps its own refusal code: an error here surfaces as that decoder's existing code.
use serde::de::{Deserialize, Deserializer, Error, MapAccess, Visitor};
use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

/// A map whose keys are unique AFTER parsing, so two spellings of one key are one key twice.
/// The `unique_secret_map` pattern of vault/mod.rs, generic over key and value.
pub(crate) fn unique_map<'de, D, K, V>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    unique_map_at_most(deserializer, usize::MAX)
}

/// The refusal of a map with more entries than its limit; see `is_over_limit`.
pub(crate) const OVER_LIMIT: &str = "map entry limit exceeded";

/// `unique_map` that also COUNTS (NA-0788 F04/S3b N3, I06): the key after the `limit`th entry
/// refuses with `OVER_LIMIT` before its value is parsed, so at most `limit` values are built.
/// `unique_map` is this visitor with no reachable limit.
pub(crate) fn unique_map_at_most<'de, D, K, V>(
    deserializer: D,
    limit: usize,
) -> Result<BTreeMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    struct Unique<K, V>(usize, PhantomData<(K, V)>);
    impl<'de, K, V> Visitor<'de> for Unique<K, V>
    where
        K: Deserialize<'de> + Ord,
        V: Deserialize<'de>,
    {
        type Value = BTreeMap<K, V>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a map with unique keys")
        }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut entries = BTreeMap::new();
            while let Some(key) = map.next_key::<K>()? {
                if entries.len() == self.0 {
                    return Err(A::Error::custom(OVER_LIMIT));
                }
                let value = map.next_value::<V>()?;
                if entries.insert(key, value).is_some() {
                    return Err(A::Error::custom("duplicate map key"));
                }
            }
            Ok(entries)
        }
    }
    deserializer.deserialize_map(Unique(limit, PhantomData))
}

/// True when a JSON decode stopped at a map over its limit, so a decoder can keep its own
/// capacity code for it (serde_json renders a custom error as its message, then the position).
pub(crate) fn is_over_limit(error: &serde_json::Error) -> bool {
    error.is_data() && error.to_string().starts_with(OVER_LIMIT)
}

/// A field that must be PRESENT. Through `deserialize_with`, serde refuses an absent field
/// instead of filling an `Option` with `None`; an explicit `null` still decodes as `None`.
pub(crate) fn required<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer)
}
