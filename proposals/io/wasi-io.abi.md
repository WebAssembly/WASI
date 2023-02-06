# Types

## <a href="#stream_error" name="stream_error"></a> `stream-error`: record

  An error type returned from a stream operation. Currently this
  doesn't provide any additional information.

Size: 0, Alignment: 1

### Record Fields

# Functions

----

#### <a href="#input_stream_read" name="input_stream_read"></a> `input-stream::read` 

  Read bytes from a stream.
  
  This function returns a list of bytes containing the data that was
  read, along with a bool indicating whether the end of the stream
  was reached. The returned list will contain up to `len` bytes; it
  may return fewer than requested, but not more.
  
  Once a stream has reached the end, subsequent calls to read or
  `skip` will always report end-of-stream rather than producing more
  data.
  
  If `len` is 0, it represents a request to read 0 bytes, which should
  always succeed, assuming the stream hasn't reached its end yet, and
  return an empty list.
  
  The len here is a `u64`, but some callees may not be able to allocate
  a buffer as large as that would imply.
  FIXME: describe what happens if allocation fails.
##### Params

- <a href="#input_stream_read.self" name="input_stream_read.self"></a> `self`: handle<input-stream>
- <a href="#input_stream_read.len" name="input_stream_read.len"></a> `len`: `u64`
##### Results

- result<(list<`u8`>, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#input_stream_skip" name="input_stream_skip"></a> `input-stream::skip` 

  Skip bytes from a stream.
  
  This is similar to the `read` function, but avoids copying the
  bytes into the instance.
  
  Once a stream has reached the end, subsequent calls to read or
  `skip` will always report end-of-stream rather than producing more
  data.
  
  This function returns the number of bytes skipped, along with a bool
  indicating whether the end of the stream was reached. The returned
  value will be at most `len`; it may be less.
##### Params

- <a href="#input_stream_skip.self" name="input_stream_skip.self"></a> `self`: handle<input-stream>
- <a href="#input_stream_skip.len" name="input_stream_skip.len"></a> `len`: `u64`
##### Results

- result<(`u64`, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#output_stream_write" name="output_stream_write"></a> `output-stream::write` 

  Write bytes to a stream.
  
  This function returns a `u64` indicating the number of bytes from
  `buf` that were written; it may be less than the full list.
##### Params

- <a href="#output_stream_write.self" name="output_stream_write.self"></a> `self`: handle<output-stream>
- <a href="#output_stream_write.buf" name="output_stream_write.buf"></a> `buf`: list<`u8`>
##### Results

- result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#output_stream_write_repeated" name="output_stream_write_repeated"></a> `output-stream::write-repeated` 

  Write multiple zero bytes to a stream.
  
  This function returns a `u64` indicating the number of zero bytes
  that were written; it may be less than `len`.
##### Params

- <a href="#output_stream_write_repeated.self" name="output_stream_write_repeated.self"></a> `self`: handle<output-stream>
- <a href="#output_stream_write_repeated.len" name="output_stream_write_repeated.len"></a> `len`: `u64`
##### Results

- result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#output_stream_splice" name="output_stream_splice"></a> `output-stream::splice` 

  Read from one stream and write to another.
  
  This function returns the number of bytes transferred; it may be less
  than `len`.
##### Params

- <a href="#output_stream_splice.self" name="output_stream_splice.self"></a> `self`: handle<output-stream>
- <a href="#output_stream_splice.src" name="output_stream_splice.src"></a> `src`: handle<input-stream>
- <a href="#output_stream_splice.len" name="output_stream_splice.len"></a> `len`: `u64`
##### Results

- result<(`u64`, `bool`), [`stream-error`](#stream_error)>

