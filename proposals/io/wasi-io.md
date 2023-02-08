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

# Import interface `wasi-io`

## Types

## <a href="#pollable" name="pollable"></a> `pollable`: [`pollable`](#pollable)


Size: 4, Alignment: 4

## <a href="#stream_error" name="stream_error"></a> `stream-error`: record

An error type returned from a stream operation. Currently this
doesn't provide any additional information.

Size: 0, Alignment: 1

### Record Fields

## <a href="#output_stream" name="output_stream"></a> `output-stream`: `u32`

An output bytestream. In the future, this will be replaced by handle
types.

This conceptually represents a `stream<u8, _>`. It's temporary
scaffolding until component-model's async features are ready.

And at present, it is a `u32` instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.

Size: 4, Alignment: 4

## <a href="#input_stream" name="input_stream"></a> `input-stream`: `u32`

An input bytestream. In the future, this will be replaced by handle
types.

This conceptually represents a `stream<u8, _>`. It's temporary
scaffolding until component-model's async features are ready.

And at present, it is a `u32` instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.

Size: 4, Alignment: 4

## Functions

----

#### <a href="#read" name="read"></a> `read` 

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

- <a href="#read.this" name="read.this"></a> `this`: [`input-stream`](#input_stream)
- <a href="#read.len" name="read.len"></a> `len`: `u64`
##### Results

- <a href="#read.result0" name="read.result0"></a> `result0`: result<(list<`u8`>, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#skip" name="skip"></a> `skip` 

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

- <a href="#skip.this" name="skip.this"></a> `this`: [`input-stream`](#input_stream)
- <a href="#skip.len" name="skip.len"></a> `len`: `u64`
##### Results

- <a href="#skip.result0" name="skip.result0"></a> `result0`: result<(`u64`, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#subscribe_read" name="subscribe_read"></a> `subscribe-read` 

Create a `pollable` which will resolve once either the specified stream has bytes
available to read or the other end of the stream has been closed.
##### Params

- <a href="#subscribe_read.this" name="subscribe_read.this"></a> `this`: [`input-stream`](#input_stream)
##### Results

- <a href="#subscribe_read.result0" name="subscribe_read.result0"></a> `result0`: [`pollable`](#pollable)

----

#### <a href="#drop_input_stream" name="drop_input_stream"></a> `drop-input-stream` 

Dispose of the specified `input-stream`, after which it may no longer
be used.
##### Params

- <a href="#drop_input_stream.this" name="drop_input_stream.this"></a> `this`: [`input-stream`](#input_stream)

----

#### <a href="#write" name="write"></a> `write` 

Write bytes to a stream.

This function returns a `u64` indicating the number of bytes from
`buf` that were written; it may be less than the full list.
##### Params

- <a href="#write.this" name="write.this"></a> `this`: [`output-stream`](#output_stream)
- <a href="#write.buf" name="write.buf"></a> `buf`: list<`u8`>
##### Results

- <a href="#write.result0" name="write.result0"></a> `result0`: result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#write_zeroes" name="write_zeroes"></a> `write-zeroes` 

Write multiple zero bytes to a stream.

This function returns a `u64` indicating the number of zero bytes
that were written; it may be less than `len`.
##### Params

- <a href="#write_zeroes.this" name="write_zeroes.this"></a> `this`: [`output-stream`](#output_stream)
- <a href="#write_zeroes.len" name="write_zeroes.len"></a> `len`: `u64`
##### Results

- <a href="#write_zeroes.result0" name="write_zeroes.result0"></a> `result0`: result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#splice" name="splice"></a> `splice` 

Read from one stream and write to another.

This function returns the number of bytes transferred; it may be less
than `len`.
##### Params

- <a href="#splice.this" name="splice.this"></a> `this`: [`output-stream`](#output_stream)
- <a href="#splice.src" name="splice.src"></a> `src`: [`input-stream`](#input_stream)
- <a href="#splice.len" name="splice.len"></a> `len`: `u64`
##### Results

- <a href="#splice.result0" name="splice.result0"></a> `result0`: result<(`u64`, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#subscribe" name="subscribe"></a> `subscribe` 

Create a `pollable` which will resolve once either the specified stream is ready
to accept bytes or the other end of the stream has been closed.
##### Params

- <a href="#subscribe.this" name="subscribe.this"></a> `this`: [`output-stream`](#output_stream)
##### Results

- <a href="#subscribe.result0" name="subscribe.result0"></a> `result0`: [`pollable`](#pollable)

----

#### <a href="#drop_output_stream" name="drop_output_stream"></a> `drop-output-stream` 

Dispose of the specified `output-stream`, after which it may no longer
be used.
##### Params

- <a href="#drop_output_stream.this" name="drop_output_stream.this"></a> `this`: [`output-stream`](#output_stream)

