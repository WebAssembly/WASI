# Functions

----

#### <a href="#get_random_bytes" name="get_random_bytes"></a> `get-random-bytes` 

  Return `len` cryptographically-secure pseudo-random bytes.
  
  This function must produce data from an adaquately seeded
  cryptographically-secure pseudo-random number generator (CSPRNG), so it
  must not block, from the perspective of the calling program, and the
  returned data is always unpredictable.
  
  This function must always return fresh pseudo-random data. Deterministic
  environments must omit this function, rather than implementing it with
  deterministic data.
##### Params

- <a href="#get_random_bytes.len" name="get_random_bytes.len"></a> `len`: `u64`
##### Results

- list<`u8`>

----

#### <a href="#get_random_u64" name="get_random_u64"></a> `get-random-u64` 

  Return a cryptographically-secure pseudo-random `u64` value.
  
  This function returns the same type of pseudo-random data as
  `get-random-bytes`, represented as a `u64`.
##### Results

- `u64`

----

#### <a href="#insecure_random" name="insecure_random"></a> `insecure-random` 

  Return a 128-bit value that may contain a pseudo-random value.
  
  The returned value is not required to be computed from a CSPRNG, and may
  even be entirely deterministic. Host implementatations are encouraged to
  provide pseudo-random values to any program exposed to attacker-controlled
  content, to enable DoS protection built into many languages' hash-map
  implementations.
  
  This function is intended to only be called once, by a source language
  to initialize Denial Of Service (DoS) protection in its hash-map
  implementation.
  
  # Expected future evolution
  
  This will likely be changed to a value import, to prevent it from being
  called multiple times and potentially used for purposes other than DoS
  protection.
##### Results

- (`u64`, `u64`)

