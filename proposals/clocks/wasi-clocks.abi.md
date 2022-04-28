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

##### Params

- <a href="#monotonic_clock_now.self" name="monotonic_clock_now.self"></a> `self`: handle<monotonic-clock>
##### Results

- <a href="#monotonic_clock_now.result" name="monotonic_clock_now.result"></a> `result`: [`instant`](#instant)

----

#### <a href="#monotonic_clock_resolution" name="monotonic_clock_resolution"></a> `monotonic-clock::resolution` 

##### Params

- <a href="#monotonic_clock_resolution.self" name="monotonic_clock_resolution.self"></a> `self`: handle<monotonic-clock>
##### Results

- <a href="#monotonic_clock_resolution.result" name="monotonic_clock_resolution.result"></a> `result`: [`instant`](#instant)

----

#### <a href="#wall_clock_now" name="wall_clock_now"></a> `wall-clock::now` 

##### Params

- <a href="#wall_clock_now.self" name="wall_clock_now.self"></a> `self`: handle<wall-clock>
##### Results

- <a href="#wall_clock_now.result" name="wall_clock_now.result"></a> `result`: [`datetime`](#datetime)

----

#### <a href="#wall_clock_resolution" name="wall_clock_resolution"></a> `wall-clock::resolution` 

##### Params

- <a href="#wall_clock_resolution.self" name="wall_clock_resolution.self"></a> `self`: handle<wall-clock>
##### Results

- <a href="#wall_clock_resolution.result" name="wall_clock_resolution.result"></a> `result`: [`datetime`](#datetime)

