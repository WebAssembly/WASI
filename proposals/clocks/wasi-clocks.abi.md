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
##### Result

- [`instant`](#instant)

----

#### <a href="#monotonic_clock_resolution" name="monotonic_clock_resolution"></a> `monotonic-clock::resolution` 

  Query the resolution of the clock.
##### Params

- <a href="#monotonic_clock_resolution.self" name="monotonic_clock_resolution.self"></a> `self`: handle<monotonic-clock>
##### Result

- [`instant`](#instant)

----

#### <a href="#monotonic_clock_new_timer" name="monotonic_clock_new_timer"></a> `monotonic-clock::new-timer` 

  This creates a new `monotonic-timer` with the given starting time. It will
  count down from this time until it reaches zero.
##### Params

- <a href="#monotonic_clock_new_timer.self" name="monotonic_clock_new_timer.self"></a> `self`: handle<monotonic-clock>
- <a href="#monotonic_clock_new_timer.initial" name="monotonic_clock_new_timer.initial"></a> `initial`: [`instant`](#instant)
##### Result

- handle<monotonic-timer>

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
##### Result

- [`datetime`](#datetime)

----

#### <a href="#wall_clock_resolution" name="wall_clock_resolution"></a> `wall-clock::resolution` 

  Query the resolution of the clock.
  
  The nanoseconds field of the output is always less than 1000000000.
##### Params

- <a href="#wall_clock_resolution.self" name="wall_clock_resolution.self"></a> `self`: handle<wall-clock>
##### Result

- [`datetime`](#datetime)

----

#### <a href="#monotonic_timer_current" name="monotonic_timer_current"></a> `monotonic-timer::current` 

  Returns the amount of time left before this timer reaches zero.
##### Params

- <a href="#monotonic_timer_current.self" name="monotonic_timer_current.self"></a> `self`: handle<monotonic-timer>
##### Result

- [`instant`](#instant)

----

#### <a href="#monotonic_timer_expiration" name="monotonic_timer_expiration"></a> `monotonic-timer::expiration` 

  Returns a future that completes when the timer reaches zero.
##### Params

- <a href="#monotonic_timer_expiration.self" name="monotonic_timer_expiration.self"></a> `self`: handle<monotonic-timer>
##### Result

- future<`unit`>

