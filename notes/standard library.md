# ov 
- [library](https://en.wikipedia.org/wiki/Library_(computing) "Library (computing)") made available across [implementations](https://en.wikipedia.org/wiki/Programming_language_implementation "Programming language implementation") of a [programming language](https://en.wikipedia.org/wiki/Programming_language)
	- i.e. a programming language is usually a spec first and there could be multiple implementations
	- e.g. there are several implementations for Python and C
	- in contrast, rust comes with the implementation first
- Every implementation of the language should provide these components
- typically there are os-dependent(hide details of the underlying OS, like files, threads, networks, etc.)
- to write an OS, we can not rely on OS so we can't use std!

## Rust's std
- provides
	- panic handling
	- stack unwinding
	- types like `Vec` and `Option`
-  `libcore` is a part of `libstd`
	- the dependency-free(and thus OS-independent) part