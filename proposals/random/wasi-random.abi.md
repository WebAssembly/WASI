# Functions

----

#### <a href="#get_random_bytes" name="get_random_bytes"></a> `get-random-bytes` 

  Return `len` random bytes.
  
  This function must produce data from an adaquately seeded CSPRNG, so it
  must not block, and the returned data is always unpredictable.
  
  Deterministic environments must omit this function, rather than
  implementing it with deterministic data.
##### Params

- <a href="#get_random_bytes.len" name="get_random_bytes.len"></a> `len`: `u32`
##### Results

- list<`u8`>

----

#### <a href="#get_random_u64" name="get_random_u64"></a> `get-random-u64` 

  Return a random `u64` value.
  
  This function must produce data from an adaquately seeded CSPRNG, so it
  must not block, and the returned data is always unpredictable.
  
  Deterministic environments must omit this function, rather than
  implementing it with deterministic data.
##### Results

- `u64`

