//! QSLFRESH v1 -- the local freshness checkpoint (C07 T6.5; DOC-CAN-003 C07-01).
//!
//! ```text
//! magic "QSLFRESH" @0+8 | version u16 BE = 1 @8+2 | profile u8 @10+1 | vault_id @11+32
//! generation u64 BE @43+8 | digest d @51+32 | anchor A @83+32 | HMAC-SHA-256(key, 0..115) @115+32
//! ```
//!
//! 147 bytes, no trailing bytes. The key is the vault payload's `checkpoint_mac_key` (C07 T6.1
//! F5); this module borrows it and never copies, stores or prints it (RULING_F04C07P R8).
//! Semantic comparisons (vault_id against the vault's, generation against the blob's) are the
//! recovery classifier's (S4), not the codec's.

use super::ProtectionMode;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::io::{self, Read};

type HmacSha256 = Hmac<Sha256>;

pub(crate) const MAGIC: &[u8; 8] = b"QSLFRESH";
pub(crate) const VERSION: u16 = 1;
pub(crate) const LEN: usize = 147;
/// I06: the reader is bounded at ONE byte past the format, so a longer file is seen (and refused
/// as trailing) without ever reading more of it.
pub(crate) const MAX_READ: usize = LEN + 1;

const VERSION_AT: usize = 8;
const PROFILE_AT: usize = 10;
const VAULT_ID_AT: usize = 11;
const GENERATION_AT: usize = 43;
const DIGEST_AT: usize = 51;
const ANCHOR_AT: usize = 83;
const MAC_AT: usize = 115;

/// The authenticated content of a checkpoint. Holds no key and no tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Checkpoint {
    pub(crate) mode: ProtectionMode,
    pub(crate) vault_id: [u8; 32],
    pub(crate) generation: u64,
    pub(crate) digest: [u8; 32],
    pub(crate) anchor: [u8; 32],
}

/// C07 ER3 (freshness_checkpoint_corrupt).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CorruptKind {
    Length,
    Trailing,
    Magic,
    Mac,
}

/// C07 ER4 (freshness_checkpoint_unsupported).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnsupportedKind {
    Version,
    Profile,
    ProfileDisagreesWithMode,
}

/// Typed internal outcomes; the client-code spellings are S6's (RULING_F04C07P R6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CheckpointError {
    Corrupt(CorruptKind),
    Unsupported(UnsupportedKind),
    /// The bounded read itself failed (not a verdict on the bytes).
    Read(io::ErrorKind),
}

fn keyed(mac_key: &[u8; 32]) -> HmacSha256 {
    // HMAC takes a key of any length (RFC 2104 sec 2); hmac 0.12's new_from_slice never errs.
    match HmacSha256::new_from_slice(mac_key) {
        Ok(mac) => mac,
        Err(_) => unreachable!(),
    }
}

pub(crate) fn encode(ck: &Checkpoint, mac_key: &[u8; 32]) -> [u8; LEN] {
    let mut out = [0u8; LEN];
    out[..VERSION_AT].copy_from_slice(MAGIC);
    out[VERSION_AT..PROFILE_AT].copy_from_slice(&VERSION.to_be_bytes());
    out[PROFILE_AT] = ck.mode.profile_byte();
    out[VAULT_ID_AT..GENERATION_AT].copy_from_slice(&ck.vault_id);
    out[GENERATION_AT..DIGEST_AT].copy_from_slice(&ck.generation.to_be_bytes());
    out[DIGEST_AT..ANCHOR_AT].copy_from_slice(&ck.digest);
    out[ANCHOR_AT..MAC_AT].copy_from_slice(&ck.anchor);
    let mut mac = keyed(mac_key);
    mac.update(&out[..MAC_AT]);
    out[MAC_AT..].copy_from_slice(&mac.finalize().into_bytes());
    out
}

/// I06: read at most [`MAX_READ`] bytes, BEFORE any parse. The underlying reader is never asked
/// for a 149th byte, whatever the file's length.
pub(crate) fn read_bounded<R: Read>(reader: R) -> io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(MAX_READ);
    reader.take(MAX_READ as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

pub(crate) fn decode_from<R: Read>(
    reader: R,
    mac_key: &[u8; 32],
    expected_mode: ProtectionMode,
) -> Result<Checkpoint, CheckpointError> {
    let bytes = read_bounded(reader).map_err(|e| CheckpointError::Read(e.kind()))?;
    decode(&bytes, mac_key, expected_mode)
}

/// THE CHECK ORDER (sealed in SEALED_EXPECTATION_S3.md sec 3):
/// 1 length (Corrupt Length / Trailing); 2 magic (Corrupt Magic); 3 version (Unsupported Version);
/// 4 profile in {1, 2} (Unsupported Profile); 5 the MAC, constant-time (Corrupt Mac);
/// 6 the AUTHENTIC profile against `expected_mode` (Unsupported ProfileDisagreesWithMode);
/// 7 the remaining fields. Nothing past magic/version/profile is used before step 5, and a
/// tampered profile byte is therefore reported as corruption, never as a mode disagreement.
pub(crate) fn decode(
    bytes: &[u8],
    mac_key: &[u8; 32],
    expected_mode: ProtectionMode,
) -> Result<Checkpoint, CheckpointError> {
    use CheckpointError::{Corrupt, Unsupported};
    if bytes.len() < LEN {
        return Err(Corrupt(CorruptKind::Length));
    }
    if bytes.len() > LEN {
        return Err(Corrupt(CorruptKind::Trailing));
    }
    if &bytes[..VERSION_AT] != MAGIC {
        return Err(Corrupt(CorruptKind::Magic));
    }
    if bytes[VERSION_AT..PROFILE_AT] != VERSION.to_be_bytes() {
        return Err(Unsupported(UnsupportedKind::Version));
    }
    let Some(mode) = ProtectionMode::from_profile_byte(bytes[PROFILE_AT]) else {
        return Err(Unsupported(UnsupportedKind::Profile));
    };
    let mut mac = keyed(mac_key);
    mac.update(&bytes[..MAC_AT]);
    mac.verify_slice(&bytes[MAC_AT..])
        .map_err(|_| Corrupt(CorruptKind::Mac))?;
    if mode != expected_mode {
        return Err(Unsupported(UnsupportedKind::ProfileDisagreesWithMode));
    }
    let mut generation = [0u8; 8];
    generation.copy_from_slice(&bytes[GENERATION_AT..DIGEST_AT]);
    Ok(Checkpoint {
        mode,
        vault_id: field32(&bytes[VAULT_ID_AT..GENERATION_AT]),
        generation: u64::from_be_bytes(generation),
        digest: field32(&bytes[DIGEST_AT..ANCHOR_AT]),
        anchor: field32(&bytes[ANCHOR_AT..MAC_AT]),
    })
}

fn field32(slice: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    out.copy_from_slice(slice);
    out
}

#[cfg(test)]
mod tests {
    use super::super::test_support::*;
    use super::*;

    /// reference_vectors.out, "VEC-2 qslfresh_147".
    const VEC2: &str = "51534c4652455348000101030a11181f262d343b424950575e656c737a81888f969da4abb2b9c0c7ced5dc\
                        010203040506070855a301aebdce4592501a1dd97da76d445a66d0bac8d84975e78a3c09e10d3b86\
                        dd937666ccb22001dbd8d46bf6d5d12de2cab0fb6bfd8e9678f9dbb4b54a79c9\
                        5693b773172531398e4d77d6d70b1467b7ef144f1170d0e52c8b30012059c70e";

    fn vec2_fields() -> Checkpoint {
        Checkpoint {
            mode: ProtectionMode::LocalCheckpoint,
            vault_id: syn_vault_id(),
            generation: SYN_GENERATION,
            digest: hex32(VEC1_D),
            anchor: hex32(VEC1_A),
        }
    }

    fn valid(mode: ProtectionMode) -> [u8; LEN] {
        encode(
            &Checkpoint {
                mode,
                ..vec2_fields()
            },
            &syn_mac_key(),
        )
    }

    const LOCAL: ProtectionMode = ProtectionMode::LocalCheckpoint;

    fn corrupt(kind: CorruptKind) -> Result<Checkpoint, CheckpointError> {
        Err(CheckpointError::Corrupt(kind))
    }

    fn unsupported(kind: UnsupportedKind) -> Result<Checkpoint, CheckpointError> {
        Err(CheckpointError::Unsupported(kind))
    }

    /// Delivers `src` and counts every byte it hands out.
    struct CountingReader {
        src: Vec<u8>,
        delivered: usize,
    }

    impl Read for CountingReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let rest = &self.src[self.delivered..];
            let n = rest.len().min(buf.len());
            buf[..n].copy_from_slice(&rest[..n]);
            self.delivered += n;
            Ok(n)
        }
    }

    #[test]
    fn vec2_encode_matches_reference() {
        let expected = hex(VEC2);
        assert_eq!(expected.len(), LEN);
        assert_eq!(
            encode(&vec2_fields(), &syn_mac_key()).as_slice(),
            expected.as_slice(),
            "QSLFRESH bytes differ from the reference script"
        );
        assert_eq!(decode(&expected, &syn_mac_key(), LOCAL), Ok(vec2_fields()));
    }

    #[test]
    fn rt1_decode_inverts_encode() {
        for mode in [ProtectionMode::LocalCheckpoint, ProtectionMode::Tpm] {
            for generation in [0, 1, 0x1_0000_0001, SYN_GENERATION, u64::MAX - 1, u64::MAX] {
                let ck = Checkpoint {
                    mode,
                    generation,
                    ..vec2_fields()
                };
                let bytes = encode(&ck, &syn_mac_key());
                assert_eq!(decode(&bytes, &syn_mac_key(), mode), Ok(ck));
            }
        }
    }

    #[test]
    fn ck1_short_file_is_corrupt_length() {
        let bytes = valid(LOCAL);
        for len in [146, 0] {
            let got = decode_from(&bytes[..len], &syn_mac_key(), LOCAL);
            assert_eq!(got, corrupt(CorruptKind::Length), "len {len}");
        }
    }

    #[test]
    fn ck2_long_file_is_corrupt_trailing_and_read_is_bounded() {
        let mut long = valid(LOCAL).to_vec();
        long.push(0);
        assert_eq!(long.len(), 148);
        assert_eq!(
            decode_from(long.as_slice(), &syn_mac_key(), LOCAL),
            corrupt(CorruptKind::Trailing)
        );
        let mut src = valid(LOCAL).to_vec();
        src.resize(4096, 0x5a);
        let mut reader = CountingReader { src, delivered: 0 };
        assert_eq!(
            decode_from(&mut reader, &syn_mac_key(), LOCAL),
            corrupt(CorruptKind::Trailing)
        );
        assert_eq!(
            reader.delivered, MAX_READ,
            "the reader must stop at 148 bytes"
        );
    }

    #[test]
    fn ck3_wrong_magic_is_corrupt_magic() {
        let mut bytes = valid(LOCAL);
        bytes[0] ^= 0x20;
        assert_eq!(
            decode(&bytes, &syn_mac_key(), LOCAL),
            corrupt(CorruptKind::Magic)
        );
    }

    #[test]
    fn ck4_version_2_is_unsupported_version() {
        let mut bytes = valid(LOCAL);
        bytes[VERSION_AT..PROFILE_AT].copy_from_slice(&2u16.to_be_bytes());
        assert_eq!(
            decode(&bytes, &syn_mac_key(), LOCAL),
            unsupported(UnsupportedKind::Version)
        );
    }

    #[test]
    fn ck5_unknown_profile_is_unsupported_profile() {
        for profile in [3u8, 0] {
            let mut bytes = valid(LOCAL);
            bytes[PROFILE_AT] = profile;
            assert_eq!(
                decode(&bytes, &syn_mac_key(), LOCAL),
                unsupported(UnsupportedKind::Profile),
                "profile {profile}"
            );
        }
    }

    #[test]
    fn ck6_profile_disagreeing_with_mode_is_unsupported() {
        let tpm = valid(ProtectionMode::Tpm);
        assert_eq!(
            decode(&tpm, &syn_mac_key(), LOCAL),
            unsupported(UnsupportedKind::ProfileDisagreesWithMode)
        );
        let local = valid(LOCAL);
        assert_eq!(
            decode(&local, &syn_mac_key(), ProtectionMode::Tpm),
            unsupported(UnsupportedKind::ProfileDisagreesWithMode)
        );
    }

    #[test]
    fn ck6b_tampered_profile_byte_is_corrupt_mac() {
        let mut bytes = valid(LOCAL);
        bytes[PROFILE_AT] = ProtectionMode::Tpm.profile_byte();
        assert_eq!(
            decode(&bytes, &syn_mac_key(), LOCAL),
            corrupt(CorruptKind::Mac)
        );
    }

    #[test]
    fn mac1_flipped_bit_in_each_region_is_corrupt_mac() {
        for (region, at) in [
            ("vault_id", VAULT_ID_AT + 5),
            ("generation", GENERATION_AT + 7),
            ("digest", DIGEST_AT),
            ("anchor", ANCHOR_AT + 31),
            ("tag", MAC_AT + 16),
        ] {
            let mut bytes = valid(LOCAL);
            bytes[at] ^= 0x01;
            assert_eq!(
                decode(&bytes, &syn_mac_key(), LOCAL),
                corrupt(CorruptKind::Mac),
                "{region}"
            );
        }
    }

    #[test]
    fn mac2_other_key_is_corrupt_mac() {
        let bytes = valid(LOCAL);
        let mut other = syn_mac_key();
        other[0] ^= 0x80;
        assert_eq!(decode(&bytes, &other, LOCAL), corrupt(CorruptKind::Mac));
    }

    /// SR-20: the dependency consumed and checked against RFC 4231 sec 4.2-4.5, 4.7, 4.8
    /// (case 5, the truncated output, is not a property this module uses).
    #[test]
    fn hmac_sha256_rfc4231_cases() {
        let long_key = vec![0xaa; 131];
        let cases: [(&str, Vec<u8>, &[u8], &str); 6] = [
            (
                "4.2",
                vec![0x0b; 20],
                b"Hi There",
                "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7",
            ),
            (
                "4.3",
                b"Jefe".to_vec(),
                b"what do ya want for nothing?",
                "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843",
            ),
            (
                "4.4",
                vec![0xaa; 20],
                &[0xdd; 50],
                "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe",
            ),
            (
                "4.5",
                (1..=25).collect(),
                &[0xcd; 50],
                "82558a389a443c0ea4cc819899f2083a85f0faa3e578f8077a2e3ff46729665b",
            ),
            (
                "4.7",
                long_key.clone(),
                b"Test Using Larger Than Block-Size Key - Hash Key First",
                "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54",
            ),
            (
                "4.8",
                long_key,
                b"This is a test using a larger than block-size key and a larger than \
                  block-size data. The key needs to be hashed before being used by the \
                  HMAC algorithm.",
                "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2",
            ),
        ];
        for (section, key, data, expected) in cases {
            let mut mac = HmacSha256::new_from_slice(&key).expect("any key length");
            mac.update(data);
            assert_eq!(
                mac.finalize().into_bytes().as_slice(),
                hex(expected).as_slice(),
                "RFC 4231 sec {section}"
            );
            let mut mac = HmacSha256::new_from_slice(&key).expect("any key length");
            mac.update(data);
            assert!(
                mac.verify_slice(&hex(expected)).is_ok(),
                "verify, sec {section}"
            );
        }
    }
}
