# WatIDL

WatIDL is an IDL for webassembly. Its goal is to be able to fully describe the import and export capabilities of webassembly modules while being intuitive to read and reason about. It inherits the webassembly text formats syntax and attempts to closely follow and fit into the webassembly ecosystem. WatIDL should used as a simple syntax for describing the interfaces along with WebIDL bindings to describe how the different data structures are formed and received by a Wasm module.

# Differences from WAT
While WatIDL follow close to WAT there are some differences

## Types
WatIDL uses the following primal types

```
intType := "i8" | "i6" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64";
floatType ::= "f32" | "f64" 
primType ::= "null" | "bool" | "string" | "data" | intTypes | floatType;
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
instead of having "module"s WatIDL has "interfaces" which has nearly the same syntax as [modules](https://webassembly.github.io/spec/core/text/modules.html). An interfaces "exports" are to be imported by a module using the interface and the interfaces "imports" are to be exported by the module using the interface. Since imports are to be provided by the module using the interface the have no module namespace associated with them.  

```
(interface $a
  ;; the module using this interface must export "memory"
  (import "memory" (memory)) 
  (global $a (export "a_global") (mut i32) (i32.const -2))
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

### Object Oriented
An interface is also a first class type

```
(interface $b)

(interface $a
  (func (export "factory")
    (result $the_result $b)
  )
)
```

### inheritance
An interface can extend a base interface inheriting its imports and exports

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
### static
A static function is one that does has no associated context. All non static function must bind to a context 

for example the following interface
```
(interface $a
  (static func (export "static")
    (result u64)
  )

  (func (export "not-static")
    (result u64)
  )
)
```
may have bindings that result in the following imports
```
(import "a" "static" (func (result i64)))
(import "a" "non-static" (func (param $this i32) (result i64)))
```

## Bindings
Watidl reuse the core of [webidl bindings](https://github.com/WebAssembly/webidl-bindings/blob/master/proposals/webidl-bindings/Explainer.md) to specify how to interact with the various complex types. 
