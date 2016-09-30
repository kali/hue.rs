# philipshue

A Rust library and some command line tools for manipulating Philips Hue lights.


[![Build Status](https://travis-ci.org/Orangenosecom/philipshue.svg?branch=master)](https://travis-ci.org/Orangenosecom/hue.rs) [![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/Orangenosecom/philipshue?branch=master&svg=true)](https://ci.appveyor.com/project/Orangenosecom/philipshue)

## Features
 - discover bridge by querying Philips Hue website *
 - list lights with their state
 - simple actions on lights (on, off, bri/hue/sat, transition time)
 - simple CLI utils for docs and tests :)


## Building
When building on you might encounter problems with OpenSSL. You may have to manually tell Rust where OpenSSL is located.

On macOS:
``` bash
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
```
On Windows:
``` batch
set OPENSSL_INCLUDE_DIR=C:\OpenSSL\include
set OPENSSL_LIB_DIR=C:\OpenSSL\lib
```
And use OpenSSL-1_0_1u from http://slproweb.com/products/Win32OpenSSL.html



## TODO

Discover using upnp rather than relying on the website *
