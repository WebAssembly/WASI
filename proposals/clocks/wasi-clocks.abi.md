# Types

## <a href="#instant" name="instant"></a> `instant`: `u64`

  A timestamp in nanoseconds.

Size: 8, Alignment: 8

## <a href="#datetime" name="datetime"></a> `datetime`: record

  A time and date in seconds plus nanoseconds.

Size: 16, Alignment: 8

### Record Fields

- <a href="datetime.seconds" name="datetime.seconds"></a> [`seconds`](#datetime.seconds): `u64`


- <a href="datetime.nanoseconds" name="datetime.nanoseconds"></a> [`nanoseconds`](#datetime.nanoseconds): `u32`


# Functions

----

#### <a href="#monotonic_clock_now" name="monotonic_clock_now"></a> `monotonic-clock::now` 

  Read the current value of the clock.
  
  As this the clock is monotonic, calling this function repeatedly will produce
  a sequence of non-decreasing values.
##### Params

- <a href="#monotonic_clock_now.self" name="monotonic_clock_now.self"></a> `self`: handle<monotonic-clock>
##### Results

- <a href="#monotonic_clock_now." name="monotonic_clock_now."></a> ``: [`instant`](#instant)

----

#### <a href="#monotonic_clock_resolution" name="monotonic_clock_resolution"></a> `monotonic-clock::resolution` 

  Query the resolution of the clock.
##### Params

- <a href="#monotonic_clock_resolution.self" name="monotonic_clock_resolution.self"></a> `self`: handle<monotonic-clock>
##### Results

- <a href="#monotonic_clock_resolution." name="monotonic_clock_resolution."></a> ``: [`instant`](#instant)

----

#### <a href="#wall_clock_now" name="wall_clock_now"></a> `wall-clock::now` 

  Read the current value of the clock.
  
  As this the clock is not monotonic, calling this function repeatedly will
  not necessarily produce a sequence of non-decreasing values.
  
  The returned timestamps represent the number of seconds since
  1970-01-01T00:00:00Z, also known as [POSIX's Seconds Since the Epoch], also
  known as [Unix Time].
  
  The nanoseconds field of the output is always less than 1000000000.
  
  [POSIX's Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16
  [Unix Time]: https://en.wikipedia.org/wiki/Unix_time
##### Params

- <a href="#wall_clock_now.self" name="wall_clock_now.self"></a> `self`: handle<wall-clock>
##### Results

- <a href="#wall_clock_now." name="wall_clock_now."></a> ``: [`datetime`](#datetime)

----

#### <a href="#wall_clock_resolution" name="wall_clock_resolution"></a> `wall-clock::resolution` 

  Query the resolution of the clock.
  
  The nanoseconds field of the output is always less than 1000000000.
##### Params

- <a href="#wall_clock_resolution.self" name="wall_clock_resolution.self"></a> `self`: handle<wall-clock>
##### Results

- <a href="#wall_clock_resolution." name="wall_clock_resolution."></a> ``: [`datetime`](#datetime)

