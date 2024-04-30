# fmod-rs

Safe rust bindings to the FMOD sound engine.
This crate tries to be as rusty and low-cost as possible, without comprimising on any APIs.
Certain APIs, such as loading banks from a pointer, are marked as unsafe, but are still available for use.

Most documentation is copied directly from the FMOD docs, however some information (such as parameter values) are excluded.
Please refer to the FMOD documentation for more usage information.

# Memory management & Copy types

All FMOD objects are Copy, Clone, Send and Sync because it's possible to have multiple references to the same object. (e.g. loading a bank and then retrieving it by its path)
Unfortunately this means that FMOD objects are not automatically released when they go out of scope, and must be manually released with the `release` method.

In my experience FMOD should be able to prevent use-after-free errors after an object is released. My experience is probably wrong though.
This is a bit like the `magnus` crate, where there are a lot of unsafe invariants related to the Ruby GC that should be marked as unsafe but would result in almost every single interaction with Ruby being unsafe.
I've elected to mark release as safe but it's up to you to ensure that you're not using an object after it's been released.

# String types
fmod-rs aims to be as zero-cost as possible, and as such, it uses UTF-8 C strings from the `lanyard` crate as its string type.
This means that all FMOD functions take a `&Utf8CStr` instead of a `&str` or `&CStr`. 
`&Utf8CStr` is pretty cheap to construct (and can even be done statically with the `c!` macro), so this should not be a problem.

When FMOD returns a string, it will always return a `Utf8CString` (the owned version of `Utf8CStr`) because it's difficult to encode lifetime requirements of FMOD strings.

This applies to structs like `fmod::studio::AdvancedSettings` which store C strings. 
Converting structs like `AdvancedSettings` to their FFI equivalents is done by reference as to not pass ownership of the string to FMOD. 

When converting from an FFI struct to something like `AdvancedSettings`, the C string pointer is cloned into a `Utf8CString`. 
*This is unsafe* as there are no guarantees that the pointer is valid or that the string is null-terminated and UTF-8.
Luckily all FMOD functions return UTF-8 strings so this isn't really a problem in practice.

# Userdata

TODO

# Differences to libfmod and fmod
libfmod is automatically generated from the FMOD documentation and is full of mistakes. It also does not provide an abstraction over userdata or callbacks.