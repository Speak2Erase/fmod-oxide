# fmod-oxide

Safe rust bindings to the FMOD sound engine.
This crate tries to be as rusty and low-cost as possible, without comprimising on any APIs.
Certain APIs, such as loading banks from a pointer, are marked as unsafe, but are still available for use.

Most documentation is copied directly from the FMOD docs, however some information (such as parameter values) are excluded.
Please refer to the FMOD documentation for more usage information.

# Memory management & Copy types

All FMOD objects are Copy, Clone, Send and Sync because it's possible to have multiple references to the same object. (e.g. loading a bank and then retrieving it by its path)
There are a lot of use-cases where you may want to fetch something (like a bank) and never use it again.
Implementing `Drop` to automatically release things would go against that particular use-case, so this crate opts to have manual `release()` methods instead.

*Most* types FMOD will prevent use-after-frees because it returns handles. There are a few times where that is not the case though.
<!-- TODO investigate what is and isnt safe to release -->
This is a bit like the `magnus` crate, where there are a lot of unsafe invariants related to the Ruby GC that should be marked as unsafe but would result in almost every single interaction with Ruby being unsafe.
I've elected to mark release as safe but it's up to you to ensure that you're not using an object after it's been released.

# String types
fmod-oxide aims to be as zero-cost as possible, and as such, it uses UTF-8 C strings from the `lanyard` crate as its string type.
This means that all FMOD functions take a `&Utf8CStr` instead of a `&str` or `&CStr`. 
`&Utf8CStr` is pretty cheap to construct (and can even be done statically with the `c!` macro), so this should not be a problem.

When FMOD returns a string, it will always return a `Utf8CString` (the owned version of `Utf8CStr`) because it's difficult to encode lifetime requirements of FMOD strings.

This applies to structs like `fmod::studio::AdvancedSettings` which store C strings. 
Converting structs like `AdvancedSettings` to their FFI equivalents is done by reference as to not pass ownership of the string to FMOD. 

When converting from an FFI struct to something like `AdvancedSettings`, the C string pointer is cloned into a `Utf8CString`. 
*This is unsafe* as there are no guarantees that the pointer is valid or that the string is null-terminated and UTF-8.
Luckily all FMOD functions return UTF-8 strings so this isn't really a problem in practice.

# Userdata

Userdata is really, really, really hard to make safe bindings for, because any libary that wants to will need to clean up userdata whenever the FMOD object it is attached to is released.
Unfortunately, there's quite a lot of cases where it's not possible to do that. 
`EventDescription` and `EventInstance` is a pretty good example- you can set a callback when events are released to clean up their userdata. 
You can also set up a callback when banks are unloaded as well to also clean up their userdata. 
That callback would be a perfect place to clean up userdata on `EventDescription` *however* you can't access the `EventDescription`s of a bank when that callback is fired.

Pretty much the only option here would be to either a) require the user to manually release userdata or b) leak memory.
Neither of these are good.

The only solution I can currently think of would be to store a map of all FMOD objects in memory and their userdata, and then every so often check if every FMOD object in it has been released. 
If it has been, we release the associated userdata as well.
This solution works best with a mark and sweep GC, which Rust does not have. We could solve somewhat this by doing this check in `System::update`.
That would make `System::update`  *really* expensive- it would have an additional `O(n)` complexity added to it, which goes against the purpose of this crate.

I'm still thinking on how to solve this issue, but I'll probably put this behind a feature gate and fall back to `*mut c_void` getters and setters if it is disabled.
Not the approach I would *like*, but oh well.

If there was an easy way to enforce that a `T` is pointer sized and needs no `Drop` (at compile time) then I could use the approach I was going for early on in this crate and just transmute the `T` to a `*mut c_void`.
(See [this commit](https://github.com/Speak2Erase/fmod-oxide/tree/a14876da32ce5df5b14673c118f09da6fec17544).)

# Differences to other crates
[libfmod](https://github.com/lebedec/libfmod) is automatically generated from the FMOD documentation and is full of mistakes. (Wrong parameter types in functions, accepting &str instead of &[u8], etc)
It also does not provide an abstraction over userdata or callbacks.

[rust-fmod](https://github.com/GuillaumeGomez/rust-fmod) is outdated, has no studio bindings, and has major safety holes (userdata takes an `&'a mut T` and does no type checking, System creation functions are not marked as unsafe, etc)

[fmod-rs](https://github.com/CAD97/fmod-rs)
I'll be honest, I wasn't aware of this crate until recently. It's missing studio bindings and is designed to be used with bevy. 
There's a couple decisions (like a reference counted Handle type) that are interesting but aren't zero cost.
If my crate doesn't work for you, it's definitely worth checking out!
