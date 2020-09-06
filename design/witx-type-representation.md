# Witx Type Representation
In Witx, each type corresponds to a lower-level type. For example, the `size` type resolves to a `u32`, the `errno` type resolves to an `Enum(u16)`, and more. Detailed below is each type, and what they resolve to in memory.

# u8/u16/u32/u64/s64
These are C-style `uint8_t`s, `uint16_t`s, `uint32_t`s, `uint64_t`s, and `int64_t`s. In the .NET world, they may be more recognizable as an `byte` (in C# this is an unsigned `byte` if you're coming from Java), `ushort`, `uint`, `ulong`, and `long`. These are typically little-endian, although it may not be necessary to know that.

# Enum(T)
An `Enum(T)` is, unlike a Rust Enum(T), simply is a `T`. However, the value of T can only be one of the specific variants of the enum. This type lends itself to describing when something can only be one of the enumerations in the group (for example, in a group of Dogs and Cats, you may have an enum representing either a Dog or a Cat).

```
errno: Enum(u32)
+-----------------+-----------------+-----------------+-----------------+
| 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 1 0 1 0 1 |
+-----------------+-----------------+-----------------+-----------------+
^ fault
```

(`clockid` despite only representing 4 values is an `Enum(u32)`. This is for ABI compatibility.)

# Flags(T)
A `Flags(T)` datatype takes up exactly a `T` in memory, similar to `Enum(T)`. However, each variant of a `Flags(T)` will take up exactly one bit in the data. This allows the usage of bitwise AND, bitwise OR, and bitwise NOT operators to combine, check, or exclude specific values in the flag very easily. This type lends itself to describing capabilities.

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
A `Struct` is a type that takes up some contiguous amount of blocks in memory, with each field taking up a specific amount of reserved bytes. Interpreting the bytes as one of the types in Witx will yield a usable value.

```
iovec

buf: Pointer<u8> @ offset 0
buf_len: size @ offset 4

+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
| 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 |
+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
^buf                                                                    ^buf_len
```

The `Size` of a `Struct` refers to how many contiguous bytes it takes up in memory.

The `Alignment` of a `Struct` refers to <X>.

# Union
A `Union` is a type which uses `tag_size` bytes to determine which variant of the union the data will be. The data is simply inserted as is with whatever type it may be.

```
subscription_u

+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
| 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 |
+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
^ padding due to the alignment of the union                                                                                   ^tag_size

cont. 32 bytes for the union's data
+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
| 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 |
+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
| 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 |
+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
| 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 |
+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
```

# Pointer<T> and Array<T>
A `Pointer<T>` and `Array<T>` are both just `Pointer<T>`s. A `Pointer<T>`'s size is guaranteed to be 8 bytes, but if on a 32 bit architecture it will only be 4 bytes. The data stored in one of these types is exactly enough to point to some data in RAM. When the pointer is dereferenced, the data stored at that location will be a contiguous amount of `T`s. // how to get length? is there one for ArrayT?

```
Pointer<u8>

+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
| 0 0 0 0 0 1 0 0 | 0 0 1 0 1 1 0 0 | 0 0 0 1 1 1 0 0 | 0 0 1 1 0 1 0 0 | 1 0 0 1 0 0 0 0 | 0 0 0 0 0 1 0 0 | 0 0 0 0 1 0 0 1 | 0 0 0 1 0 0 0 0 |
+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+-----------------+
^ makes up a value, which when dereferenced leads to another point in RAM with the actual data
```
