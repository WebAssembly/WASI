# WASI Clocks Timezone API

# `wasi-timezone`
```wit
default interface wasi-timezone {
```

## `datetime`
```wit
/// A time and date in seconds plus nanoseconds.
///
/// TODO: Use the definition from the monotonic clock API instead of defining our own copy.
record datetime {
    seconds: u64,
    nanoseconds: u32,
}
```

## `timezone`
```wit
/// A timezone.
///
/// In timezones that recognize daylight saving time, also known as daylight
/// time and summer time, the information returned from the functions varies
/// over time to reflect these adjustments.
// TODO(resource timezone {)
type timezone = u32
```

## `display`
```wit
    /// Return information needed to display the given `datetime`. This includes
    /// the UTC offset, the time zone name, and a flag indicating whether
    /// daylight saving time is active.
    ///
    /// If the timezone cannot be determined for the given `datetime`, return a
    /// `timezone-display` for `UTC` with a `utc-offset` of 0 and no daylight
    /// saving time.
    display: func(this: timezone, when: datetime) -> timezone-display
```

## `utc-offset`
```wit
    /// The same as `display`, but only return the UTC offset.
    utc-offset: func(this: timezone, when: datetime) -> s32
```

## `drop-timezone`
```wit
/// Dispose of the specified input-stream, after which it may no longer
/// be used.
// TODO(} /* resource timezone */)
drop-timezone: func(this: timezone)
```

## `timezone-display`
```wit
/// Information useful for displaying the timezone of a specific `datetime`.
///
/// This information may vary within a single `timezone` to reflect daylight
/// saving time adjustments.
record timezone-display {
```

## `utc-offset`
```wit
    /// The number of seconds difference between UTC time and the local time of
    /// the timezone.
    ///
    /// The returned value will always be less than 86400 which is the number of
    /// seconds in a day (24*60*60).
    ///
    /// In implementations that do not expose an actual time zone, this should
    /// return 0.
    utc-offset: s32,
```

## `name`
```wit
    /// The abbreviated name of the timezone to display to a user. The name `UTC`
    /// indicates Coordinated Universal Time. Otherwise, this should reference
    /// local standards for the name of the time zone.
    ///
    /// In implementations that do not expose an actual time zone, this should be
    /// the string `UTC`.
    ///
    /// In time zones that do not have an applicable name, a formatted
    /// representation of the UTC offset may be returned, such as `-04:00`.
    name: string,
```

## `in-daylight-saving-time`
```wit
    /// Whether daylight saving time is active.
    ///
    /// In implementations that do not expose an actual time zone, this should
    /// return false.
    in-daylight-saving-time: bool,
```

```wit
}
```

```wit
}
```
