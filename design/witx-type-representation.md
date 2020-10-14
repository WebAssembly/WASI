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

# Enum(T)

An `Enum(T)` is simply is a `T`. However, the value of T can only be one of the
specific variants of the enum. This type lends itself to describing when
something can only be one of the enumerations in the group (for example, in a
group of Dogs and Cats, you may have an enum representing either a Dog or a
Cat).

```
errno: Enum(u32)
+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x15 |
+------+------+------+------+
^ fault
```

(`clockid` despite only representing 4 values is an `Enum(u32)`. This is
primarily for ABI compatibility, and future-proofing.)

# Flags(T)

A `Flags(T)` datatype takes up exactly a `T` in memory, similar to `Enum(T)`.
However, each variant of a `Flags(T)` will take up exactly one bit in the data.
This allows the usage of bitwise AND, bitwise OR, and bitwise NOT operators to
combine, check, or exclude specific values in the flag very easily. This type
lends itself to describing capabilities.

```
oflags: Flags(u16)

+-----------------+-----------------+
| 0 1 1 0 0 0 0 0 | 0 0 0 0 0 0 0 0 |
+-----------------+-----------------+
  ^ ^ ^ ^
  | | | trunc
  | | excl
  | directory
  creat
```

# Struct

A `Struct` is a type that takes up some contiguous amount of memory, with each
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

The `Size` of a `Struct` refers to how many contiguous bytes it takes up in
memory.

The `Alignment` of a `Struct` refers to <X>.

# Union

A `Union` is a type which uses `tag_size` bytes to determine which variant of
the union the data will be. The data is simply inserted as is with whatever
type it may be.

```
subscription_u

+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
^ padding due to the alignment of the union      ^tag_size

cont. 32 bytes for the union's data
+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
```

# Pointer<T> and Array<T>

A `Pointer<T>` and `Array<T>` are both just `Pointer<T>`s. A `Pointer<T>`'s
size is guaranteed to be 8 bytes, but if on a 32 bit architecture it will only
use 4 of the 8 bytes. The pointers themselves are 32 bit, as wasm32 is the only
ABI currently implemented. In the future when the specification for wasm64 is
done, that will change. The data stored in one of these types is exactly enough
to point to some data in RAM. When the pointer is dereferenced, the data stored
at that location will be an unknown contiguous amount of `T`s.

```
Pointer<u8>

+------+------+------+------+------+------+------+------+
| 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 |
+------+------+------+------+------+------+------+------+
^ a number, that represents another position in RAM that leads to the data.
```
