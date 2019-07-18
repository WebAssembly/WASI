# Virtualization

Virtualization in the context of WASI interfaces is the ability for a Webassembly module to implement a given interface and consumer module to be able to use that implement transparently. 

Furthermore there are two classes of virtualization; static and dynamic. Static virtualization is done before the initialization of a Wasm store and dynamic at runtime.  

## Usecase

Roughly taken from [here](https://pdfs.semanticscholar.org/4cce/9abb177cc58c199e13b82d498f37010c2bfc.pdf). Lets say a user wants to encrypt a text file they are editing. One way this could be accomplished is by having composable services. An encryption module could export the interface for a directory that would encrypt any file written to it. Lets consider how this could be done with in the framework of WASI. At runtime the encryption module would given a root directory file descriptor in which it would write encrypted text to, it would also would return a wrapped file descriptor of a directory  that would be given to the text editor to write in. This means any WASI `fd` related calls that the text editor makes would handled by the encryption module which would then interact base system. Lets look next at a WASI program might accomplish virtualizing the file descriptor interface.

## Virtualization with GC

The one possibility would be to use an object oriented approach. This is relies on the [function references](https://github.com/WebAssembly/function-references/blob/master/proposals/function-references/Overview.md) and [GC](https://github.com/WebAssembly/gc/blob/master/proposals/gc/Overview.md) proposals.  

We might have the following type definition for a `fd` and its related functions.
```
(type $open_func (func (param i32) (param i32) ... ))
(type $close_func)
...

(type $fd (struct 
  (field $open (ref $open_func))
  (field $close (ref $close_func))
))

```

`func.bind` from function references proposals would be used by the programs implementing the interface to create proxy functions.

```
(func $fd-open-proxy (param $_fd (ref $fd)) (param i32) (param i32) ... )
  (local $open_ref (ref $open_func))
   ...
  (call_ref
    ... ;; other params 
    (get_field $open_func $open (get_local $_fd)
  )
  
)

(func mk-fd-open-proxy (param (ref $fd) (result (ref $open_func)))
  (func.bind $open_func (local.get 0) (func.ref $fd-open-proxy))
)
```

In this scenario no functions are imported, only types. While this is one possible future we don't currently have this functionality. While it maybe be possible to polyfill things like `func.bind` using tables, other things such as full GC would more complicated and may create overhead.

### Dynamic Dispatching - Without GC 

Assuming the function reference proposal, another way to implement dynamic dispatching currently would be to create a primitive polyfill for structs. An `fd` would then be represented as a stuct who's fields contained all the associated functions

```
struct.create(fields: u32) -> anyRef
```

```
struct.set(index: i32, func: funcRef)
```

```
struct.get(stuct_ref: anyRef, index: u32) -> funcRef
```
