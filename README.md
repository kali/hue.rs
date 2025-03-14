# Rust library for Philips Hue

[![Build Status](https://travis-ci.org/kali/hue.rs.svg?branch=master)](https://travis-ci.org/kali/hue.rs)

## Features
 - discover bridge by querying philips hue website or using UPnP
 - list lights with their state
 - simple actions on lights (on, off, bri/hue/sat, transition time)
 - simple CLI utils for docs and tests :)

## Licencing

Originally, this crate being a week-end one-shot hack, I released it under WTFPL license. My intent was
to allow anybody to use this software under the terms of a very permissive license, while minimizing the
licensing discussions and arguments.

A couple of years later, it appears that the strategy failed it second objective as  WTFPL regularly
raise the kind of issues it was meant to avoid.

So, this software is now released under the following licensinc scheme:

Apache 2.0/MIT/WTFPL

All original work licensed under either of

    Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
    WTFPL (https://en.wikipedia.org/wiki/WTFPL)

at your option.

Additionally, in order to minimize the discussion around WTFPL wording, only "MIT OR Apache-2.0" will be
tagged in Cargo.toml. The intent here is that WTFPL does not appear on its own or in combination in
license tracking systems.
