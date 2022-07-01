# Functions

----

#### <a href="#getrandom" name="getrandom"></a> `getrandom` 

  Return `len` random bytes.
  
  This function must produce data from an adaquately seeded CSPRNG, so it
  must not block, and the returned data is always unpredictable.
  
  Deterministic environments must omit this function, rather than
  implementing it with deterministic data.
##### Params

- <a href="#getrandom.len" name="getrandom.len"></a> `len`: `u32`
##### Result

- list<`u8`>

