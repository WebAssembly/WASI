# Witx Type Representation

In Witx, each type corresponds to a lower-level type. For example, the `size`
type resolves to a `u32`, the `errno` type resolves to an `Enum(u16)`, and
more. Detailed below is each type, and what they correspond to in the
WebAssembly type system.

## u8/u16/u32/u64/s64

These are C-style `uint8_t`s, `uint16_t`s, `uint32_t`s, `uint64_t`s, and
`int64_t`s. More information about the signedness-aware integer types [can be
found here](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md#integers).

The endian-ness of these types are **little endian**, as _all_ types in the WebAssembly
type system are little endian. [See here for more information](https://github.com/WebAssembly/design/issues/786#issuecomment-244548105).

# Record

A `Record` is a type that takes up some contiguous amount of memory, with each
field taking up a specific amount of reserved bytes. Interpreting the bytes as
one of the types in Witx will yield a usable value.

```
iovec

buf: Pointer<u8> @ offset 0
buf_len: size @ offset 4

+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
^buf                        ^buf_len
```

The `Size` of a `Record` refers to how many contiguous bytes it takes up in
memory.

The `Alignment` of a `Record` refers to the byte boundary that the record must
be on in memory. For example, the above `iovec` could only ever get allocated
to a memory address that is divisible by 4, because the alignment is 4.

# Variant

A `Variant` is a type which uses some bits to determine which variant of
the variant the data will be. The data is simply inserted as is with whatever
type it may be.

```
errno: Variant
+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x15 |
+------+------+------+------+
^ fault
```

```
subscription_u

+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
^ padding due to the alignment of the variant    ^tag_size

cont. 32 bytes for the variant's data
+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
```

# Pointer<T> and List<T>

A `Pointer<T>` and `List<T>` are both just `Pointer<T>`s. A `Pointer<T>`'s
size is guaranteed to be 8 bytes, but if on a 32 bit architecture it will only
use 4 of the 8 bytes. The pointers themselves are 32 bit, as wasm32 is the only
ABI currently implemented. In the future when the specification for wasm64 is
done, that will change. The data stored in one of these types is exactly enough
to point to some data in RAM. When the pointer is dereferenced, the data stored
at that location will be an unknown contiguous amount of `T`s.

```
Pointer<u8>

+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+
^ a number, that represents another position in RAM that leads to the data.
```
