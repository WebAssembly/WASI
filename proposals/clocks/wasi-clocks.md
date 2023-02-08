# Import interface `wasi-poll`

## Types

## <a href="#pollable" name="pollable"></a> `pollable`: `u32`

A "pollable" handle.

This is conceptually represents a `stream<_, _>`, or in other words,
a stream that one can wait on, repeatedly, but which does not itself
produce any data. It's temporary scaffolding until component-model's
async features are ready.

And at present, it is a `u32` instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.

`pollable` lifetimes are not automatically managed. Users must ensure
that they do not outlive the resource they reference.

Size: 4, Alignment: 4

## Functions

----

#### <a href="#poll_oneoff" name="poll_oneoff"></a> `poll-oneoff` 

Poll for completion on a set of pollables.

The "oneoff" in the name refers to the fact that this function must do a
linear scan through the entire list of subscriptions, which may be
inefficient if the number is large and the same subscriptions are used
many times. In the future, it may be accompanied by an API similar to
Linux's `epoll` which allows sets of subscriptions to be registered and
made efficiently reusable.

Note that the return type would ideally be `list<bool>`, but that would
be more difficult to polyfill given the current state of `wit-bindgen`.
See https://github.com/bytecodealliance/preview2-prototyping/pull/11#issuecomment-1329873061
for details.  For now, we use zero to mean "not ready" and non-zero to
mean "ready".
##### Params

- <a href="#poll_oneoff.in" name="poll_oneoff.in"></a> `in`: list<[`pollable`](#pollable)>
##### Results

- <a href="#poll_oneoff.result0" name="poll_oneoff.result0"></a> `result0`: list<`u8`>

# Import interface `wasi-monotonic-clock`

## Types

## <a href="#pollable" name="pollable"></a> `pollable`: [`pollable`](#pollable)


Size: 4, Alignment: 4

## <a href="#monotonic_clock" name="monotonic_clock"></a> `monotonic-clock`: `u32`

A monotonic clock is a clock which has an unspecified initial value, and
successive reads of the clock will produce non-decreasing values.

It is intended for measuring elapsed time.

Size: 4, Alignment: 4

## <a href="#instant" name="instant"></a> `instant`: `u64`

A timestamp in nanoseconds.

Size: 8, Alignment: 8

## Functions

----

#### <a href="#now" name="now"></a> `now` 

Read the current value of the clock.

The clock is monotonic, therefore calling this function repeatedly will produce
a sequence of non-decreasing values.
##### Params

- <a href="#now.this" name="now.this"></a> `this`: [`monotonic-clock`](#monotonic_clock)
##### Results

- <a href="#now.result0" name="now.result0"></a> `result0`: [`instant`](#instant)

----

#### <a href="#resolution" name="resolution"></a> `resolution` 

Query the resolution of the clock.
##### Params

- <a href="#resolution.this" name="resolution.this"></a> `this`: [`monotonic-clock`](#monotonic_clock)
##### Results

- <a href="#resolution.result0" name="resolution.result0"></a> `result0`: [`instant`](#instant)

----

#### <a href="#subscribe" name="subscribe"></a> `subscribe` 

Create a `pollable` which will resolve once the specified time has been reached.
##### Params

- <a href="#subscribe.this" name="subscribe.this"></a> `this`: [`monotonic-clock`](#monotonic_clock)
- <a href="#subscribe.when" name="subscribe.when"></a> `when`: [`instant`](#instant)
- <a href="#subscribe.absolute" name="subscribe.absolute"></a> `absolute`: `bool`
##### Results

- <a href="#subscribe.result0" name="subscribe.result0"></a> `result0`: [`pollable`](#pollable)

----

#### <a href="#drop_monotonic_clock" name="drop_monotonic_clock"></a> `drop-monotonic-clock` 

Dispose of the specified `monotonic-clock`, after which it may no longer
be used.
##### Params

- <a href="#drop_monotonic_clock.this" name="drop_monotonic_clock.this"></a> `this`: [`monotonic-clock`](#monotonic_clock)

# Import interface `wasi-wall-clock`

## Types

## <a href="#wall_clock" name="wall_clock"></a> `wall-clock`: `u32`

A wall clock is a clock which measures the date and time according to some
external reference.

External references may be reset, so this clock is not necessarily
monotonic, making it unsuitable for measuring elapsed time.

It is intended for reporting the current date and time for humans.

Size: 4, Alignment: 4

## <a href="#datetime" name="datetime"></a> `datetime`: record

A time and date in seconds plus nanoseconds.

Size: 16, Alignment: 8

### Record Fields

- <a href="datetime.seconds" name="datetime.seconds"></a> [`seconds`](#datetime.seconds): `u64`
  
  
- <a href="datetime.nanoseconds" name="datetime.nanoseconds"></a> [`nanoseconds`](#datetime.nanoseconds): `u32`
  
  
## Functions

----

#### <a href="#now" name="now"></a> `now` 

Read the current value of the clock.

This clock is not monotonic, therefore calling this function repeatedly will
not necessarily produce a sequence of non-decreasing values.

The returned timestamps represent the number of seconds since
1970-01-01T00:00:00Z, also known as [POSIX's Seconds Since the Epoch], also
known as [Unix Time].

The nanoseconds field of the output is always less than 1000000000.

[POSIX's Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16
[Unix Time]: https://en.wikipedia.org/wiki/Unix_time
##### Params

- <a href="#now.this" name="now.this"></a> `this`: [`wall-clock`](#wall_clock)
##### Results

- <a href="#now.result0" name="now.result0"></a> `result0`: [`datetime`](#datetime)

----

#### <a href="#resolution" name="resolution"></a> `resolution` 

Query the resolution of the clock.

The nanoseconds field of the output is always less than 1000000000.
##### Params

- <a href="#resolution.this" name="resolution.this"></a> `this`: [`wall-clock`](#wall_clock)
##### Results

- <a href="#resolution.result0" name="resolution.result0"></a> `result0`: [`datetime`](#datetime)

----

#### <a href="#drop_wall_clock" name="drop_wall_clock"></a> `drop-wall-clock` 

Dispose of the specified `wall-clock`, after which it may no longer
be used.
##### Params

- <a href="#drop_wall_clock.this" name="drop_wall_clock.this"></a> `this`: [`wall-clock`](#wall_clock)

# Import interface `wasi-timezone`

## Types

## <a href="#timezone_display" name="timezone_display"></a> `timezone-display`: record

Information useful for displaying the timezone of a specific `datetime`.

This information may vary within a single `timezone` to reflect daylight
saving time adjustments.

Size: 16, Alignment: 4

### Record Fields

- <a href="timezone_display.utc_offset" name="timezone_display.utc_offset"></a> [`utc-offset`](#timezone_display.utc_offset): `u32`
  
  The number of seconds difference between UTC time and the local time of
  the timezone.
  
  The returned value will always be less than 86400 which is the number of
  seconds in a day (24*60*60).
  
  In implementations that do not expose an actual time zone, this should
  return 0.
  
- <a href="timezone_display.name" name="timezone_display.name"></a> [`name`](#timezone_display.name): `string`
  
  The abbreviated name of the timezone to display to a user. The name `UTC`
  indicates Coordinated Universal Time. Otherwise, this should reference
  local standards for the name of the time zone.
  
  In implementations that do not expose an actual time zone, this should be
  the string `UTC`.
  
  In time zones that do not have an applicable name, a formatted
  representation of the UTC offset may be returned, such as `-04:00`.
  
- <a href="timezone_display.in_daylight_saving_time" name="timezone_display.in_daylight_saving_time"></a> [`in-daylight-saving-time`](#timezone_display.in_daylight_saving_time): `bool`
  
  Whether daylight saving time is active.
  
  In implementations that do not expose an actual time zone, this should
  return false.
  
## <a href="#timezone" name="timezone"></a> `timezone`: `u32`

A timezone.

In timezones that recognize daylight saving time, also known as daylight
time and summer time, the information returned from the functions varies
over time to reflect these adjustments.

Size: 4, Alignment: 4

## <a href="#datetime" name="datetime"></a> `datetime`: record

A time and date in seconds plus nanoseconds.

TODO: Use the definition from the monotonic clock API instead of defining our own copy.

Size: 16, Alignment: 8

### Record Fields

- <a href="datetime.seconds" name="datetime.seconds"></a> [`seconds`](#datetime.seconds): `u64`
  
  
- <a href="datetime.nanoseconds" name="datetime.nanoseconds"></a> [`nanoseconds`](#datetime.nanoseconds): `u32`
  
  
## Functions

----

#### <a href="#display" name="display"></a> `display` 

Return information needed to display the given `datetime`. This includes
the UTC offset, the time zone name, and a flag indicating whether
daylight saving time is active.

If the timezone cannot be determined for the given `datetime`, return a
`timezone-display` for `UTC` with a `utc-offset` of 0 and no daylight
saving time.
##### Params

- <a href="#display.this" name="display.this"></a> `this`: [`timezone`](#timezone)
- <a href="#display.when" name="display.when"></a> `when`: [`datetime`](#datetime)
##### Results

- <a href="#display.result0" name="display.result0"></a> `result0`: [`timezone-display`](#timezone_display)

----

#### <a href="#utc_offset" name="utc_offset"></a> `utc-offset` 

The same as `display`, but only return the UTC offset.
##### Params

- <a href="#utc_offset.this" name="utc_offset.this"></a> `this`: [`timezone`](#timezone)
- <a href="#utc_offset.when" name="utc_offset.when"></a> `when`: [`datetime`](#datetime)
##### Results

- <a href="#utc_offset.result0" name="utc_offset.result0"></a> `result0`: `u32`

----

#### <a href="#drop_timezone" name="drop_timezone"></a> `drop-timezone` 

Dispose of the specified input-stream, after which it may no longer
be used.
##### Params

- <a href="#drop_timezone.this" name="drop_timezone.this"></a> `this`: [`timezone`](#timezone)

# Import interface `wasi-default-clocks`

## Types

## <a href="#monotonic_clock" name="monotonic_clock"></a> `monotonic-clock`: [`monotonic-clock`](#monotonic_clock)


Size: 4, Alignment: 4

## <a href="#wall_clock" name="wall_clock"></a> `wall-clock`: [`wall-clock`](#wall_clock)


Size: 4, Alignment: 4

## Functions

----

#### <a href="#default_monotonic_clock" name="default_monotonic_clock"></a> `default-monotonic-clock` 

Return a default monotonic clock, suitable for general-purpose application
needs.

This allocates a new handle, so applications with frequent need of a clock
handle should call this function once and reuse the handle instead of
calling this function each time.
##### Results

- <a href="#default_monotonic_clock.result0" name="default_monotonic_clock.result0"></a> `result0`: [`monotonic-clock`](#monotonic_clock)

----

#### <a href="#default_wall_clock" name="default_wall_clock"></a> `default-wall-clock` 

Return a default wall clock, suitable for general-purpose application
needs.

This allocates a new handle, so applications with frequent need of a clock
handle should call this function once and reuse the handle instead of
calling this function each time.
##### Results

- <a href="#default_wall_clock.result0" name="default_wall_clock.result0"></a> `result0`: [`wall-clock`](#wall_clock)

