# WASI Random API

WASI Random is a random data API.

It is intended to be portable at least between Unix-family platforms and
Windows.

## `getrandom`
```wit
/// Return `len` random bytes.
getrandom: function(len: u32) -> list<u8>
```
