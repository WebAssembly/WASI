# WASI Random API

WASI Random is a random data API.

It is intended to be portable at least between Unix-family platforms and
Windows.

## `getrandom`
```wit
/// Return `len` random bytes.
///
/// This function must produce data from an adaquately seeded CSPRNG, so it
/// must not block, and the returned data is always unpredictable.
///
/// Deterministic environments must omit this function, rather than
/// implementing it with deterministic data.
getrandom: function(len: u32) -> list<u8>
```

## `insecure-random`
```wit
/// A value containing 128 random bits.
///
/// This is a value import, which means it only provides one value, rather
/// than being a function that could be called multiple times. This is intented
/// to be used by source languages to initialize hash-maps without needing the
/// full `getrandom` API.
///
/// This value is not required to be computed from a CSPRNG, and may even be
/// entirely deterministic. Host implementatations are encouraged to provide
/// random values to any program exposed to attacker-controlled content, to
/// enable DoS protection built into many languages' hash-map implementations.
insecure-random: tuple<u64, u64>
```
