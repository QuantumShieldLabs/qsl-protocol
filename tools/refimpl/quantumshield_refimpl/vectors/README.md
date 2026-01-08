# Vectors

- `parse_only.json` is a non-cryptographic fixture set for canonical parser tests.
- Future vectors will include cryptographically-valid handshakes, messaging, and KT proof verification.

Implementations MUST:
- parse `wire_hex` exactly,
- reject malformed encodings (unknown flags, invalid lengths, trailing bytes),
- reproduce `expected` values for cryptographic vectors.
