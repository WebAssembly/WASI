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
##### Params

- <a href="#input_stream_read.self" name="input_stream_read.self"></a> `self`: handle<input-stream>
- <a href="#input_stream_read.len" name="input_stream_read.len"></a> `len`: `u64`
##### Results

- result<(list<`u8`>, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#input_stream_skip" name="input_stream_skip"></a> `input-stream::skip` 

  Skip bytes from a stream.
##### Params

- <a href="#input_stream_skip.self" name="input_stream_skip.self"></a> `self`: handle<input-stream>
- <a href="#input_stream_skip.len" name="input_stream_skip.len"></a> `len`: `u64`
##### Results

- result<(`u64`, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#output_stream_write" name="output_stream_write"></a> `output-stream::write` 

  Write bytes to a stream.
##### Params

- <a href="#output_stream_write.self" name="output_stream_write.self"></a> `self`: handle<output-stream>
- <a href="#output_stream_write.buf" name="output_stream_write.buf"></a> `buf`: list<`u8`>
##### Results

- result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#output_stream_write_repeated" name="output_stream_write_repeated"></a> `output-stream::write-repeated` 

  Write bytes to a stream.
##### Params

- <a href="#output_stream_write_repeated.self" name="output_stream_write_repeated.self"></a> `self`: handle<output-stream>
- <a href="#output_stream_write_repeated.byte" name="output_stream_write_repeated.byte"></a> `byte`: `u8`
- <a href="#output_stream_write_repeated.len" name="output_stream_write_repeated.len"></a> `len`: `u64`
##### Results

- result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#output_stream_splice_stream" name="output_stream_splice_stream"></a> `output-stream::splice-stream` 

  Read from one stream and write to another.
##### Params

- <a href="#output_stream_splice_stream.self" name="output_stream_splice_stream.self"></a> `self`: handle<output-stream>
- <a href="#output_stream_splice_stream.src" name="output_stream_splice_stream.src"></a> `src`: handle<input-stream>
- <a href="#output_stream_splice_stream.len" name="output_stream_splice_stream.len"></a> `len`: `u64`
##### Results

- result<(`u64`, `bool`), [`stream-error`](#stream_error)>

