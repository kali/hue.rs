# philipshue

[![Build Status](https://travis-ci.org/Orangenosecom/philipshue.svg?branch=master)](https://travis-ci.org/Orangenosecom/hue.rs) [![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/Orangenosecom/philipshue?branch=master&svg=true)](https://ci.appveyor.com/project/Orangenosecom/philipshue)

Library for interacting with the Hue API in order to manipulate Hue lights.

The goal of this library is to provide a easy way of interacting with the Hue API.

## Current features

- Discover bridge by querying the Philips Hue website
- Find all lights connected to a bridge
- Simple actions on the lights (e.g. turn on and off and setting the colour)

## Building

When building, you might encounter problems with OpenSSL.
You may have to manually tell Rust where OpenSSL is located through environment variables.
Have a look at the [README of rust-openssl][rust-openssl] for more help.

### On macOS

```bash
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
```

### On Windows

```batch
set OPENSSL_INCLUDE_DIR=C:\OpenSSL\include
set OPENSSL_LIB_DIR=C:\OpenSSL\lib
set OPENSSL_LIBS=ssleay32:libeay32
```

Install OpenSSL-1_0_1u from <http://slproweb.com/products/Win32OpenSSL.html>.
Make sure to install it in the same directory as written in the environment variables
(in the case of this example: `C:\OpenSSL\`).

[rust-openssl]: https://github.com/sfackler/rust-openssl/blob/master/README.md
