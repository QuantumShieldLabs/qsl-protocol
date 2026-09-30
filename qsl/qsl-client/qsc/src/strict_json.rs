//! Strict decoding of authenticated JSON records (NA-0788 F04/S1; C01 T1 rows 15 and 21,
//! C07 T6). A stored record is state, not an update: a repeated map key is contradictory and
//! refused, never resolved last-value-wins, and no field is filled in from its absence.
//! These helpers only parse; they never repair or rewrite input. Each decoder that uses them
//! keeps its own refusal code: an error here surfaces as that decoder's existing code.
use serde::de::{Deserialize, DeserializeSeed, Deserializer, Error, MapAccess, Visitor};
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
    unique_map_seeded(deserializer, limit, PhantomData::<V>)
}

/// `unique_map_at_most` with every value decoded through `seed`: a `PhantomData<V>` for a
/// `Deserialize` value, `b64::Seed` for a byte field (NA-0788 F04/S5). The ONE strict map visitor.
fn unique_map_seeded<'de, D, K, S>(
    deserializer: D,
    limit: usize,
    seed: S,
) -> Result<BTreeMap<K, S::Value>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Ord,
    S: DeserializeSeed<'de> + Copy,
{
    struct Unique<K, S>(usize, S, PhantomData<K>);
    impl<'de, K, S> Visitor<'de> for Unique<K, S>
    where
        K: Deserialize<'de> + Ord,
        S: DeserializeSeed<'de> + Copy,
    {
        type Value = BTreeMap<K, S::Value>;
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
                let value = map.next_value_seed(self.1)?;
                if entries.insert(key, value).is_some() {
                    return Err(A::Error::custom("duplicate map key"));
                }
            }
            Ok(entries)
        }
    }
    deserializer.deserialize_map(Unique(limit, seed, PhantomData))
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

/// A set decoded from a JSON array whose elements are unique (NA-0788 F04/S4b N2, RULING_NA0788_S1
/// R3): a repeated element is contradictory and refused, never merged into one. Order is not
/// checked; the writers emit a set in ascending order.
pub(crate) fn unique_set<'de, D, T>(
    deserializer: D,
) -> Result<std::collections::BTreeSet<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Ord,
{
    struct Unique<T>(PhantomData<T>);
    impl<'de, T> Visitor<'de> for Unique<T>
    where
        T: Deserialize<'de> + Ord,
    {
        type Value = std::collections::BTreeSet<T>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an array with unique elements")
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut elements = std::collections::BTreeSet::new();
            while let Some(element) = seq.next_element::<T>()? {
                if !elements.insert(element) {
                    return Err(A::Error::custom("duplicate set element"));
                }
            }
            Ok(elements)
        }
    }
    deserializer.deserialize_seq(Unique(PhantomData))
}

// NA-0788 F04/S5 (RBANK_F04_encoding_schema_version, C4-O7): the schema version of the four
// stored records, and the ONE codec for byte fields at rest.

/// The one new client refusal code of F04/S5 (RBANK_S5_schema_version_code; DOC-CAN-009
/// QRC-0022): a stored directional record carries a schema version this client does not know.
/// Raised as a serde message by the versioned readers below and mapped to itself by each
/// top-level decoder (`is_version_unsupported`), so an unknown version is never called tampered.
pub(crate) const RECORD_VERSION_UNSUPPORTED: &str = "directional_record_version_unsupported";
/// The schema version this client writes and reads for CapacityOwner, Transaction, Flight and
/// Disposition. A change of their fields is a new value here and a named migration, never a
/// silent reinterpretation.
pub(crate) const SCHEMA_VERSION: u64 = 1;
const SCHEMA: &str = "schema";

pub(crate) fn is_version_unsupported(error: &serde_json::Error) -> bool {
    error.is_data() && error.to_string().starts_with(RECORD_VERSION_UNSUPPORTED)
}

/// The schema version of a top-level record's text, read from the WHOLE object before any field
/// is decoded (E2's detection order: an unknown version has different fields, so strictness must
/// not see them first). Every other member is skipped unread; an absent, repeated or non-integer
/// `schema` is an error the caller reports with its existing code.
pub(crate) fn schema_of(raw: &str) -> Result<u64, serde_json::Error> {
    #[derive(serde::Deserialize)]
    struct Peek {
        schema: u64,
    }
    serde_json::from_str::<Peek>(raw).map(|peek| peek.schema)
}

/// A record that carries `schema`: its version-1 fields are a serde `remote` definition, so the
/// field strictness of S1-S4b is unchanged; this trait joins the two.
pub(crate) trait Versioned: Sized {
    fn fields<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
    fn serialize_fields<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

/// The three impls of a schema-carrying record over its version-1 field definition; `$reader`
/// is `deserialize_top` for a record the vault stores as text, `deserialize_nested` for one
/// held inside another record's map.
macro_rules! versioned_record {
    ($record:ty, $fields:ty, $reader:ident) => {
        impl $crate::strict_json::Versioned for $record {
            fn fields<'de, D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                <$fields>::deserialize(deserializer)
            }
            fn serialize_fields<S: ::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                <$fields>::serialize(self, serializer)
            }
        }
        impl ::serde::Serialize for $record {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                $crate::strict_json::serialize_versioned(self, serializer)
            }
        }
        impl<'de> ::serde::Deserialize<'de> for $record {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                $crate::strict_json::$reader(deserializer)
            }
        }
    };
}
pub(crate) use versioned_record;

/// `{"schema":1, ...fields}`: the version first, then the fields, in one object.
pub(crate) fn serialize_versioned<S: serde::Serializer, T: Versioned>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    struct Fields<'a, T>(&'a T);
    impl<T: Versioned> serde::Serialize for Fields<'_, T> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize_fields(serializer)
        }
    }
    #[derive(serde::Serialize)]
    #[serde(bound = "T: Versioned")]
    struct Record<'a, T> {
        schema: u64,
        #[serde(flatten)]
        fields: Fields<'a, T>,
    }
    serde::Serialize::serialize(
        &Record {
            schema: SCHEMA_VERSION,
            fields: Fields(value),
        },
        serializer,
    )
}

/// A top-level record (Transaction, CapacityOwner), whose text the caller has already passed
/// through `schema_of`: the `schema` member is hidden from the version-1 field definition and
/// checked once more as a backstop, so a reader that skipped the peek still refuses another
/// version (with a code that then depends on member order; the peek makes it exact).
pub(crate) fn deserialize_top<'de, D: Deserializer<'de>, T: Versioned>(
    deserializer: D,
) -> Result<T, D::Error> {
    use serde::de::value::MapAccessDeserializer;
    use serde::de::IntoDeserializer;
    use std::cell::Cell;
    struct Filtered<'s, A> {
        map: A,
        seen: &'s Cell<Option<u64>>,
    }
    impl<'de, A: MapAccess<'de>> MapAccess<'de> for Filtered<'_, A> {
        type Error = A::Error;
        fn next_key_seed<K: DeserializeSeed<'de>>(
            &mut self,
            seed: K,
        ) -> Result<Option<K::Value>, A::Error> {
            loop {
                let Some(key) = self.map.next_key::<String>()? else {
                    return Ok(None);
                };
                if key != SCHEMA {
                    return seed.deserialize(key.into_deserializer()).map(Some);
                }
                let version = self.map.next_value::<u64>()?;
                if self.seen.replace(Some(version)).is_some() {
                    return Err(A::Error::duplicate_field(SCHEMA));
                }
            }
        }
        fn next_value_seed<V: DeserializeSeed<'de>>(
            &mut self,
            seed: V,
        ) -> Result<V::Value, A::Error> {
            self.map.next_value_seed(seed)
        }
    }
    struct Top<T>(PhantomData<T>);
    impl<'de, T: Versioned> Visitor<'de> for Top<T> {
        type Value = T;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a record with a schema version")
        }
        fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<T, A::Error> {
            let seen = Cell::new(None);
            let value = T::fields(MapAccessDeserializer::new(Filtered { map, seen: &seen }))?;
            match seen.get() {
                Some(SCHEMA_VERSION) => Ok(value),
                Some(_) => Err(A::Error::custom(RECORD_VERSION_UNSUPPORTED)),
                None => Err(A::Error::missing_field(SCHEMA)),
            }
        }
    }
    deserializer.deserialize_map(Top(PhantomData))
}

/// A nested record (Flight, Disposition, held inside the Transaction's maps): its members are
/// read into a buffer of this one object's entries -- a repeated member is refused here, as at
/// every map level -- so the version is known before any field is decoded; version 1's fields
/// then decode strictly from the buffer. The buffer is one copy of this object's text, bounded by
/// the record cap the top-level decoder applied to the whole text; these two records hold no
/// key material (hashes and ciphertext only).
pub(crate) fn deserialize_nested<'de, D: Deserializer<'de>, T: Versioned>(
    deserializer: D,
) -> Result<T, D::Error> {
    struct Nested<T>(PhantomData<T>);
    impl<'de, T: Versioned> Visitor<'de> for Nested<T> {
        type Value = T;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a record with a schema version")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<T, A::Error> {
            let mut members = serde_json::Map::new();
            while let Some(key) = map.next_key::<String>()? {
                let value = map.next_value::<serde_json::Value>()?;
                if members.insert(key, value).is_some() {
                    return Err(A::Error::custom("duplicate map key"));
                }
            }
            match members.remove(SCHEMA).map(|v| v.as_u64()) {
                Some(Some(SCHEMA_VERSION)) => {}
                Some(Some(_)) => return Err(A::Error::custom(RECORD_VERSION_UNSUPPORTED)),
                Some(None) => return Err(A::Error::custom("schema version is not an integer")),
                None => return Err(A::Error::missing_field(SCHEMA)),
            }
            T::fields(serde_json::Value::Object(members)).map_err(A::Error::custom)
        }
    }
    deserializer.deserialize_map(Nested(PhantomData))
}

/// Byte fields at rest (E1; C4-O7): canonical padded STANDARD base64 in a JSON string, decoded
/// strictly. The engine refuses a length that is not a multiple of four, a missing, extra or
/// misplaced pad, a byte outside the alphabet and non-zero trailing bits (base64 0.22,
/// RequireCanonical); the text must also re-encode to itself; a fixed-size field's text length
/// is checked before anything is decoded. Every buffer that holds the bytes or their text here is
/// zeroizing (the fields include key material); the destination field and serde_json's output
/// string belong to the caller. The ONE codec for stored byte fields: a JSON number array, or any
/// other type, is refused by the enclosing decoder's existing code.
pub(crate) mod b64 {
    use super::{unique_map_seeded, Deserialize, DeserializeSeed, Deserializer, Error, Visitor};
    use base64::Engine as _;
    use serde::ser::{SerializeMap, Serializer};
    use std::collections::BTreeMap;
    use std::fmt;
    use std::marker::PhantomData;
    use zeroize::Zeroizing;

    const ENGINE: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;
    const NOT_CANONICAL: &str = "byte field is not canonical base64";

    /// The canonical text of `bytes`, in zeroizing storage.
    fn text(bytes: &[u8]) -> Zeroizing<String> {
        Zeroizing::new(ENGINE.encode(bytes))
    }
    /// The bytes `text` canonically encodes, or None. `fixed` demands exactly that many bytes,
    /// checked on the text's length before the decode and on the bytes after it.
    fn bytes(text: &str, fixed: Option<usize>) -> Option<Zeroizing<Vec<u8>>> {
        if fixed.is_some_and(|n| text.len() != n.div_ceil(3) * 4) {
            return None;
        }
        let bytes = Zeroizing::new(ENGINE.decode(text).ok()?);
        if fixed.is_some_and(|n| bytes.len() != n) {
            return None;
        }
        let again = Zeroizing::new(ENGINE.encode(bytes.as_slice()));
        (again.as_str() == text).then_some(bytes)
    }

    /// A byte field's storage form: `[u8; N]`, `Vec<u8>`, or an `Option` of one (null when None;
    /// an absent field is refused, as `required`).
    pub(crate) trait Bytes: Sized {
        fn encode<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
        fn decode<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
    }
    struct Fixed<const N: usize>;
    impl<'de, const N: usize> Visitor<'de> for Fixed<N> {
        type Value = [u8; N];
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "canonical base64 of {N} bytes")
        }
        fn visit_str<E: Error>(self, text: &str) -> Result<[u8; N], E> {
            fixed::<N, E>(text).map(|out| *out)
        }
    }
    /// The `N` bytes `text` canonically encodes, in zeroizing storage. The copy into the array is
    /// FALLIBLE (NA-0788 F04/S6, RULING_NA0788_S5b_MERGE R2 / SR-15 F4): a length the check inside
    /// `bytes` did not stop refuses as NOT_CANONICAL here; nothing in the copy can panic.
    fn fixed<const N: usize, E: Error>(text: &str) -> Result<Zeroizing<[u8; N]>, E> {
        let bytes = bytes(text, Some(N)).ok_or_else(|| E::custom(NOT_CANONICAL))?;
        <[u8; N]>::try_from(bytes.as_slice())
            .map(Zeroizing::new)
            .map_err(|_| E::custom(NOT_CANONICAL))
    }
    /// The fixed form decoded INTO zeroizing storage (NA-0788 F04/S6): for a key member of the v5
    /// vault payload, whose bytes must never rest in a plain array.
    struct FixedZeroizing<const N: usize>;
    impl<'de, const N: usize> Visitor<'de> for FixedZeroizing<N> {
        type Value = Zeroizing<[u8; N]>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "canonical base64 of {N} bytes")
        }
        fn visit_str<E: Error>(self, text: &str) -> Result<Zeroizing<[u8; N]>, E> {
            fixed::<N, E>(text)
        }
    }
    struct Variable;
    impl<'de> Visitor<'de> for Variable {
        type Value = Vec<u8>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("canonical base64")
        }
        fn visit_str<E: Error>(self, text: &str) -> Result<Vec<u8>, E> {
            let mut bytes = bytes(text, None).ok_or_else(|| E::custom(NOT_CANONICAL))?;
            // The buffer itself moves to the destination; nothing is left behind to clear.
            Ok(std::mem::take(&mut *bytes))
        }
    }
    struct Maybe<T>(PhantomData<T>);
    impl<'de, T: Bytes> Visitor<'de> for Maybe<T> {
        type Value = Option<T>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("canonical base64 or null")
        }
        fn visit_none<E: Error>(self) -> Result<Option<T>, E> {
            Ok(None)
        }
        fn visit_unit<E: Error>(self) -> Result<Option<T>, E> {
            Ok(None)
        }
        fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Option<T>, D::Error> {
            T::decode(deserializer).map(Some)
        }
    }
    impl<const N: usize> Bytes for [u8; N] {
        fn encode<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(&text(self))
        }
        fn decode<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_str(Fixed::<N>)
        }
    }
    impl Bytes for Vec<u8> {
        fn encode<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(&text(self))
        }
        fn decode<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_str(Variable)
        }
    }
    /// NA-0788 F04/S6: the ONE `Bytes` impl for a fixed field held in zeroizing storage (the v5
    /// payload's checkpoint_mac_key and nv_auth): its text is canonical base64 like any fixed field,
    /// and its bytes are decoded straight into a `Zeroizing` array.
    impl<const N: usize> Bytes for Zeroizing<[u8; N]> {
        fn encode<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(&text(self.as_slice()))
        }
        fn decode<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_str(FixedZeroizing::<N>)
        }
    }
    impl<T: Bytes> Bytes for Option<T> {
        fn encode<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                None => serializer.serialize_none(),
                Some(value) => serializer.serialize_some(&Text(value)),
            }
        }
        fn decode<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_option(Maybe(PhantomData))
        }
    }
    /// `Serialize` for a byte field, for options and map values.
    struct Text<'a, T>(&'a T);
    impl<T: Bytes> serde::Serialize for Text<'_, T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.encode(serializer)
        }
    }
    /// The seed decoding a map value as a byte field.
    pub(crate) struct Seed<T>(PhantomData<T>);
    impl<T> Clone for Seed<T> {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<T> Copy for Seed<T> {}
    impl<'de, T: Bytes> DeserializeSeed<'de> for Seed<T> {
        type Value = T;
        fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<T, D::Error> {
            T::decode(deserializer)
        }
    }

    /// `#[serde(with = "crate::strict_json::b64")]` on a field.
    pub(crate) fn serialize<S: Serializer, T: Bytes>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.encode(serializer)
    }
    pub(crate) fn deserialize<'de, D: Deserializer<'de>, T: Bytes>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        T::decode(deserializer)
    }
    /// A map whose values are byte fields (Epoch.skipped, Core.peer, Transaction.completed).
    pub(crate) fn serialize_map<S: Serializer, K: serde::Serialize, V: Bytes>(
        map: &BTreeMap<K, V>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut out = serializer.serialize_map(Some(map.len()))?;
        for (key, value) in map {
            out.serialize_entry(key, &Text(value))?;
        }
        out.end()
    }
    pub(crate) fn unique_map<'de, D, K, V>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
        K: Deserialize<'de> + Ord,
        V: Bytes,
    {
        unique_map_at_most(deserializer, usize::MAX)
    }
    pub(crate) fn unique_map_at_most<'de, D, K, V>(
        deserializer: D,
        limit: usize,
    ) -> Result<BTreeMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
        K: Deserialize<'de> + Ord,
        V: Bytes,
    {
        unique_map_seeded(deserializer, limit, Seed::<V>(PhantomData))
    }
}

#[cfg(test)]
mod f04_s5_codec_tests {
    use super::b64::Bytes;
    use base64::Engine as _;

    fn decode<T: Bytes>(text: &str) -> Result<T, serde_json::Error> {
        T::decode(&mut serde_json::Deserializer::from_str(
            &serde_json::to_string(text).unwrap(),
        ))
    }
    fn encode<T: Bytes>(value: &T) -> String {
        let mut out = Vec::new();
        value
            .encode(&mut serde_json::Serializer::new(&mut out))
            .unwrap();
        String::from_utf8(out).unwrap()
    }

    /// The secret-hygiene census as a test (E1): every engine call of the codec binds its result
    /// into zeroizing storage on the same line, and the codec is the file's only engine use.
    #[test]
    fn t_e1_every_codec_buffer_is_zeroizing() {
        let source = include_str!("strict_json.rs");
        let start = source.find("pub(crate) mod b64 {").unwrap();
        let end = start + source[start..].find("\n}\n").unwrap();
        let module = &source[start..end];
        let calls: Vec<&str> = module.lines().filter(|l| l.contains("ENGINE.")).collect();
        assert_eq!(calls.len(), 3, "{calls:?}");
        for line in &calls {
            assert!(line.contains("Zeroizing::new("), "{line}");
        }
        let product = &source[..source.find("#[cfg(test)]").unwrap()];
        let elsewhere = product.matches("ENGINE.").count() - calls.len();
        assert_eq!(elsewhere, 0);
    }

    /// Lengths 0, 1, 2 and 3 round-trip in the variable form; the fixed form demands its length.
    #[test]
    fn t_e1_codec_round_trips_and_lengths() {
        for (bytes, text) in [
            (vec![], "\"\""),
            (vec![1], "\"AQ==\""),
            (vec![1, 2], "\"AQI=\""),
            (vec![1, 2, 3], "\"AQID\""),
        ] {
            assert_eq!(encode(&bytes), text);
            assert_eq!(decode::<Vec<u8>>(&text[1..text.len() - 1]).unwrap(), bytes);
        }
        let sixteen = [7u8; 16];
        let text = base64::engine::general_purpose::STANDARD.encode(sixteen);
        assert_eq!(text.len(), 24);
        assert_eq!(decode::<[u8; 16]>(&text).unwrap(), sixteen);
        assert!(
            decode::<[u8; 32]>(&text).is_err(),
            "wrong length for the fixed form"
        );
        assert!(decode::<[u8; 16]>("").is_err());
        assert_eq!(decode::<Option<[u8; 16]>>(&text).unwrap(), Some(sixteen));
        let null: Option<[u8; 16]> =
            Bytes::decode(&mut serde_json::Deserializer::from_str("null")).unwrap();
        assert_eq!(null, None);
        assert_eq!(encode(&None::<[u8; 16]>), "null");
    }

    /// Which non-canonical forms base64 0.22's STANDARD engine refuses on its own (printed), and
    /// that the codec refuses every one of them.
    #[test]
    fn t_e1_engine_canonicality_probe() {
        let engine = base64::engine::general_purpose::STANDARD;
        let canonical = engine.encode([7u8; 16]);
        assert_eq!(canonical, "BwcHBwcHBwcHBwcHBwcHBw==");
        let forms = [
            ("no padding", "BwcHBwcHBwcHBwcHBwcHBw"),
            ("one pad short", "BwcHBwcHBwcHBwcHBwcHBw="),
            ("extra pad", "BwcHBwcHBwcHBwcHBwcHBw==="),
            ("pad inside", "BwcHBwcHBwcH=wcHBwcHBw=="),
            ("trailing garbage", "BwcHBwcHBwcHBwcHBwcHBw==A"),
            ("non-alphabet -", "BwcHBwcHBwcHBwcHBwcHB-=="),
            ("space inside", "BwcHBwcHBwcH BwcHBwcHBw=="),
            ("non-zero trailing bits", "BwcHBwcHBwcHBwcHBwcHBx=="),
            ("newline", "BwcHBwcHBwcHBwcHBwcHBw==\n"),
        ];
        for (name, form) in forms {
            let engine_alone = engine.decode(form).is_ok();
            println!("S5PROBE engine_accepts={engine_alone} form={name}");
            assert!(decode::<[u8; 16]>(form).is_err(), "{name}");
            assert!(decode::<Vec<u8>>(form).is_err(), "{name}");
        }
        assert_eq!(decode::<[u8; 16]>(&canonical).unwrap(), [7u8; 16]);
    }
}

// NA-0788 F04/S6, the S5b rider (RULING_NA0788_S5b_MERGE R2; SR-15 F4, F5, F7): the witnesses the
// S5b read found missing, on the codec and the two readers directly.
#[cfg(test)]
mod f04_s6_rider_tests {
    use super::b64::Bytes;
    use super::{deserialize_nested, deserialize_top, is_version_unsupported, Versioned};
    use base64::Engine as _;
    use zeroize::Zeroizing;

    fn decode<T: Bytes>(text: &str) -> Result<T, serde_json::Error> {
        T::decode(&mut serde_json::Deserializer::from_str(
            &serde_json::to_string(text).unwrap(),
        ))
    }
    fn encode<T: Bytes>(value: &T) -> String {
        let mut out = Vec::new();
        value
            .encode(&mut serde_json::Serializer::new(&mut out))
            .unwrap();
        String::from_utf8(out).unwrap()
    }

    /// F4: a text of the RIGHT length for a fixed field that decodes to the WRONG number of bytes --
    /// 24 characters with no pad (18 bytes) or one pad (17) for 16 bytes, 44 with no pad (33) or two
    /// pads (31) for 32 -- refuses as not canonical and never panics in the copy into the array.
    #[test]
    fn t_e1_fixed_field_length_forms_refused_without_panic() {
        let engine = base64::engine::general_purpose::STANDARD;
        let sixteen = engine.encode([7u8; 16]);
        let thirty_two = engine.encode([9u8; 32]);
        assert_eq!((sixteen.len(), thirty_two.len()), (24, 44));
        assert!(sixteen.ends_with("==") && thirty_two.ends_with('='));
        let forms16 = [
            (
                "no pad, 24 chars (18 bytes)",
                format!("{}AA", &sixteen[..22]),
            ),
            (
                "one pad, 24 chars (17 bytes)",
                format!("{}A=", &sixteen[..22]),
            ),
        ];
        let forms32 = [
            (
                "no pad, 44 chars (33 bytes)",
                format!("{}A", &thirty_two[..43]),
            ),
            (
                "two pads, 44 chars (31 bytes)",
                format!("{}==", &thirty_two[..42]),
            ),
        ];
        for (name, form) in &forms16 {
            assert_eq!(form.len(), 24, "{name}");
            let bytes = engine.decode(form).map(|b| b.len());
            println!("S6PROBE f4 form={name} engine={bytes:?}");
            assert!(decode::<[u8; 16]>(form).is_err(), "{name}");
            assert!(
                decode::<Zeroizing<[u8; 16]>>(form).is_err(),
                "{name} (zeroizing)"
            );
        }
        for (name, form) in &forms32 {
            assert_eq!(form.len(), 44, "{name}");
            let bytes = engine.decode(form).map(|b| b.len());
            println!("S6PROBE f4 form={name} engine={bytes:?}");
            assert!(decode::<[u8; 32]>(form).is_err(), "{name}");
            assert!(
                decode::<Zeroizing<[u8; 32]>>(form).is_err(),
                "{name} (zeroizing)"
            );
        }
        let err = decode::<[u8; 16]>(&forms16[0].1).unwrap_err();
        assert!(
            err.to_string()
                .starts_with("byte field is not canonical base64"),
            "{err}"
        );
        assert_eq!(decode::<[u8; 16]>(&sixteen).unwrap(), [7u8; 16]);
        assert_eq!(decode::<[u8; 32]>(&thirty_two).unwrap(), [9u8; 32]);
    }

    /// S6: the fixed form decoded into zeroizing storage round-trips, demands its length, and its
    /// text is the same canonical text as the plain fixed form's.
    #[test]
    fn t_s6_zeroizing_fixed_field_round_trips() {
        let key = Zeroizing::new([0x5au8; 32]);
        let text = encode(&key);
        assert_eq!(text, encode(&[0x5au8; 32]));
        assert_eq!(text.len(), 46);
        let back: Zeroizing<[u8; 32]> = decode(&text[1..45]).unwrap();
        assert_eq!(*back, [0x5au8; 32]);
        assert!(
            decode::<Zeroizing<[u8; 16]>>(&text[1..45]).is_err(),
            "wrong length"
        );
        assert!(decode::<Zeroizing<[u8; 32]>>("").is_err());
        let source = include_str!("strict_json.rs");
        let start = source.find("pub(crate) mod b64 {").unwrap();
        let module = &source[start..start + source[start..].find("\n}\n").unwrap()];
        let code: Vec<&str> = module
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect();
        assert_eq!(
            code.iter()
                .filter(|l| l.contains("copy_from_slice"))
                .count(),
            0,
            "the copy is fallible (F4)"
        );
        assert_eq!(module.matches("<[u8; N]>::try_from(").count(), 1);
        assert_eq!(
            module
                .matches("impl<const N: usize> Bytes for Zeroizing<[u8; N]>")
                .count(),
            1
        );
    }

    /// A probe record over the same readers the four stored records use.
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ProbeV1 {
        #[serde(with = "super::b64")]
        wire: Vec<u8>,
        slot: u32,
    }
    impl Versioned for ProbeV1 {
        fn fields<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            <Self as serde::Deserialize>::deserialize(deserializer)
        }
        fn serialize_fields<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serde::Serialize::serialize(self, serializer)
        }
    }
    fn top(text: &str) -> Result<ProbeV1, serde_json::Error> {
        deserialize_top(&mut serde_json::Deserializer::from_str(text))
    }
    fn nested(text: &str) -> Result<ProbeV1, serde_json::Error> {
        deserialize_nested(&mut serde_json::Deserializer::from_str(text))
    }

    /// F5: the top-level reader's backstop, reached DIRECTLY (no peek): schema 2 with no extra member
    /// is the version code wherever `schema` sits; the same on the real CapacityOwner through
    /// serde_json::from_str, which is the reader without CapacityOwner::decode's peek.
    #[test]
    fn t_e2_backstop_refuses_schema_2_without_extra_member() {
        assert_eq!(
            top(r#"{"schema":1,"wire":"BQ==","slot":1}"#).unwrap(),
            ProbeV1 {
                wire: vec![5],
                slot: 1
            }
        );
        for text in [
            r#"{"schema":2,"wire":"BQ==","slot":1}"#,
            r#"{"wire":"BQ==","slot":1,"schema":2}"#,
            r#"{"wire":"BQ==","schema":2,"slot":1}"#,
            r#"{"schema":0,"wire":"BQ==","slot":1}"#,
        ] {
            let err = top(text).unwrap_err();
            assert!(is_version_unsupported(&err), "{text}: {err}");
        }
        let err = top(r#"{"wire":"BQ==","slot":1}"#).unwrap_err();
        assert!(
            !is_version_unsupported(&err),
            "absent schema keeps the existing code: {err}"
        );
        let owner = serde_json::to_string(&crate::protocol_state::CapacityOwner {
            generation: 3,
            peers: Default::default(),
            entries: Default::default(),
        })
        .unwrap();
        assert!(owner.starts_with("{\"schema\":1,"), "{owner}");
        let two = owner.replacen("{\"schema\":1,", "{\"schema\":2,", 1);
        let err = serde_json::from_str::<crate::protocol_state::CapacityOwner>(&two)
            .err()
            .unwrap();
        assert!(is_version_unsupported(&err), "{err}");
        let inner = owner
            .strip_prefix("{\"schema\":1,")
            .unwrap()
            .strip_suffix('}')
            .unwrap();
        let last = format!("{{{inner},\"schema\":2}}");
        let err = serde_json::from_str::<crate::protocol_state::CapacityOwner>(&last)
            .err()
            .unwrap();
        assert!(is_version_unsupported(&err), "{last}: {err}");
        assert!(serde_json::from_str::<crate::protocol_state::CapacityOwner>(&owner).is_ok());
    }

    /// F7: the nested reader refuses a repeated NON-schema member (last-wins would read it), with
    /// the duplicate-key message every nested error maps to the record's tampered code by.
    #[test]
    fn t_e2_repeated_non_schema_nested_member_refused() {
        assert_eq!(
            nested(r#"{"schema":1,"wire":"BQ==","slot":1}"#).unwrap(),
            ProbeV1 {
                wire: vec![5],
                slot: 1
            }
        );
        for text in [
            r#"{"schema":1,"wire":"BQ==","slot":1,"wire":"BQ=="}"#,
            r#"{"schema":1,"wire":"BQ==","wire":"BQ==","slot":1}"#,
            r#"{"schema":1,"wire":"BQ==","slot":1,"slot":1}"#,
            r#"{"slot":1,"schema":1,"slot":2,"wire":"BQ=="}"#,
        ] {
            let err = nested(text).unwrap_err();
            assert!(
                err.to_string().starts_with("duplicate map key"),
                "{text}: {err}"
            );
            assert!(!is_version_unsupported(&err));
        }
        let err = nested(r#"{"schema":1,"wire":"BQ==","slot":1,"schema":1}"#).unwrap_err();
        assert!(err.to_string().starts_with("duplicate map key"), "{err}");
    }
}
