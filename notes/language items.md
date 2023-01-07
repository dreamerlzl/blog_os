# OV
- syntactically, an lang item is an attribute `#[lang = "..."]` attached to an item
- Lang items are a way for stdlib/libstd to define types, traits, functions, and other items which the compiler needs to know about
	- a mechanism to  make the compiler independent of the concrete implementation of libstd (note that it's OS-dependent)
- examples
	- `main` function is attached with `#[lang = "start"]` so it's the entry point of an executable
	- `core::Add` is attached with `#[lang = "add"]` so that the `Add:add` would be used for `x+y` 

# References
- [rust lang_items](https://doc.rust-lang.org/nightly/unstable-book/language-features/lang-items.html)