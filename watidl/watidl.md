# WatIDL :duck:

WatIDL (`/ˈwädl/`) is an IDL for webassembly. Its goal is to be able to fully describe the import and export capabilities of Webassembly modules while being intuitive to read and reason about. It inherits the Webassembly text format's syntax and attempts to closely follow and fit into the Webassembly ecosystem. WatIDL should used as a simple syntax for describing the interfaces along with WebIDL bindings to describe how the different data structures are formed and received by Wasm modules.

# Differences from WAT
While WatIDL follow close to WAT there are some differences

## Types
WatIDL uses the following primitive types

```
refTypes := "anyRef" | "funcRef"
intType := "s8" | "s6" | "s32" | "s64" | "u8" | "u16" | "u32" | "u64";
floatType ::= "f32" | "f64" 
primType ::= "null" | "bool" | "string" | "data" | intType | floatType | refTypes;
```

In addition new types can also be created using
- struct
- array
- union
- enum

The syntax for these follows the [gc proposals](https://github.com/WebAssembly/gc/blob/master/proposals/gc/Overview.md) syntax when possible.

```
(type $drinks
  (enum
    $coffee
    $juice
    $water
    $wine
  )
)

(type $baz
  (struct
    (field $foo $drinks)
    (field $bar bool)
  )
)
```

## Interface and Functions
### Imports and Exports
Instead of having "module"s WatIDL has "interfaces" which has nearly the same syntax as [modules](https://webassembly.github.io/spec/core/text/modules.html). An interfaces "exports" are to be imported by a module using the interface and the interfaces "imports" are to be exported by the module using the interface. Since imports are to be provided by the module using the interface the have no module namespace associated with them.  

```
(interface $a
  ;; the module using this interface must export "memory"
  (import "memory" (memory)) 
  (global $a (export "a_global") (mut s32) (s32.const -2))
  (func (export "a_func"))
)
```

### Results
watidl can have multiple results and results can be labeled
```
(func (export "multi-vale")
  (result $result1 u64)
  (result $result2 f32)
)
```

### Inheritance
An interface can extend a base interface inheriting its imports, functions and types

```
(interface $b
  (func (export "add")
    (param u64 u64)
    (result u64)
  )
)

(interface $a extends $b
  (func (export "sub")
    (param u64 u64)
    (result u64)
  )
)
```

### Method
The method parameter type signifies that the function expects a "this" value. A function definition can only have one `method` parameter. In the future this will enable us to have OO style bindings.

for example the following interface
```
(interface $a
  (func (export "foo")
    (method anyRef)
    (param i32)
    (result u64)
  )
)
```

## Bindings
Watidl reuse the core of [webidl bindings](https://github.com/WebAssembly/webidl-bindings/blob/master/proposals/webidl-bindings/Explainer.md) to specify how to interact with the various complex types. 
