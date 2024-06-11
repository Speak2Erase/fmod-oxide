<!--
 Copyright (c) 2024 Lily Lyons
 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# fmod-audio-sys

Low level bindgen wrappers for FMOD, like other -sys crates.

The library itself is licensed under `MPLv2` and does not include FMOD's API!
You will need to download FMOD yourself to make full use of this crate.

Currently named `fmod-audio-sys` both to distinguish itself as bindings for the FMOD audio engine and because `fmod-sys` is already taken :p

# Usage

Add this crate as a dependency:
```toml
[dependencies]
fmod-audio-sys = "2.220.0"
```

You'll need to download and install FMOD's API.
On Windows, all you need to do is run the installer- everything else is handled for you.

On other platforms, you'll need to place FMOD somewhere (usually your binary's root) and set `FMOD_SYS_FMOD_DIRECTORY` via `.cargo/config.toml`.