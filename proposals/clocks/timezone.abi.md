# Types

## <a href="#datetime" name="datetime"></a> `datetime`: record

  A time and date in seconds plus nanoseconds.
  
  TODO: Use the definition from the monotonic clock API instead of defining our own copy.

Size: 16, Alignment: 8

### Record Fields

- <a href="datetime.seconds" name="datetime.seconds"></a> [`seconds`](#datetime.seconds): `u64`


- <a href="datetime.nanoseconds" name="datetime.nanoseconds"></a> [`nanoseconds`](#datetime.nanoseconds): `u32`


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

# Functions

----

#### <a href="#timezone_display" name="timezone_display"></a> `timezone::display` 

  Return information needed to display the given `datetime`. This includes
  the UTC offset, the time zone name, and a flag indicating whether
  daylight saving time is active.
  
  If the timezone cannot be determined for the given `datetime`, return a
  `timezone-display` for `UTC` with a `utc-offset` of 0 and no daylight
  saving time.
##### Params

- <a href="#timezone_display.self" name="timezone_display.self"></a> `self`: handle<timezone>
- <a href="#timezone_display.when" name="timezone_display.when"></a> `when`: [`datetime`](#datetime)
##### Results

- [`timezone-display`](#timezone_display)

----

#### <a href="#timezone_utc_offset" name="timezone_utc_offset"></a> `timezone::utc-offset` 

  The same as `display`, but only return the UTC offset.
##### Params

- <a href="#timezone_utc_offset.self" name="timezone_utc_offset.self"></a> `self`: handle<timezone>
- <a href="#timezone_utc_offset.when" name="timezone_utc_offset.when"></a> `when`: [`datetime`](#datetime)
##### Results

- `u32`

