# MO-Libary (WIP)

[![dependency status](https://deps.rs/repo/github/arma3modorganizer/MO-Libary/status.svg)](https://deps.rs/repo/github/arma3modorganizer/MO-Libary)
[![Travis CI](https://travis-ci.org/arma3modorganizer/MO-Libary.svg?branch=master)](https://travis-ci.org/arma3modorganizer/MO-Libary)
[![Build status](https://ci.appveyor.com/api/projects/status/d39clo2lta1qbv08?svg=true)](https://ci.appveyor.com/project/Scarjit/mo-libary)
[![Crates.io](https://img.shields.io/crates/v/a3mo_lib)](https://crates.io/crates/a3mo_lib)
[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/arma3modorganizer/MO-Libary/blob/master/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/arma3modorganizer/MO-Libary/badge.svg?branch=master)](https://coveralls.io/github/arma3modorganizer/MO-Libary?branch=master)

This is the backend libary, that powers all Arma3 Mod Organizer projects.

Due to the WIP status of the project, anything can and will most likely change !

## Usage
 1. Clone the repository
 2. Use ```cargo build --all --all-targets``` to build it.
    1. If you are using rust, you can instead import it as a cargo crate:
    ```
    [dependencies]
    a3mo_lib = {path="path_to_lib_folder"}
    ```
 3. An example for FFI bindings will be uploaded, once the Libary is no longer WIP.

## TODO
 - Finalize *Build*
 - Add Features:
   - List
   - Remove
 - Add Delta Patching
 - Add Tests
 
 ## Licenses
 ### [rust-fasthash](https://github.com/flier/rust-fasthash)
 Rust-fasthash is developed under [Apache 2.0 License](https://github.com/flier/rust-fasthash/blob/master/LICENSE).
Changes where made to adapt it for easier use with Files.
The license file is also found under [crypto/apache_2_0](src/crypto/apache_2_0)