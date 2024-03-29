# Welcome to file-ext!

Hi, `file-ext` is a collection of utility functions to work with files. It is tested and works on Linux, macOS and Windows.

## Features
1. File
    1. Read 
    1. Read partially
    1. Does file exist
    1. Create 
    1. Read file, if it doesn't exist create and write to file
    1. Write to file
    1. Delete 
    1. Copy
2. Directory
    1. Create
    2. Does directory exist
    3. Delete directory
3. Path
    1. Absolute path to [working directory](https://en.wikipedia.org/wiki/Working_directory)
    2. Get OS dependent path separator ('/' on Unix, '\\' on Windows)
    3. Get path to OS temporary folder ('/tmp' on Linux and macOS, 'C:\\Users\\**_username_**\\AppData\\Local\\Temp' on Windows
4. [Symlink](https://en.wikipedia.org/wiki/Symbolic_link)
    1. Does symlink exist
    2. Get path symlink points to 
    3. Create symlink
5. User
    1. Get the name of the user who is running the process
    2. Get [domain](https://en.wikipedia.org/wiki/Windows_domain) user belongs to (available only on Windows) 

## Configuration
No additional configuration.

## Demo
Take a look at [mod.rs](https://github.com/bohdaq/file-ext/blob/main/src/lib.rs)

## Documentation
Take a look at [mod.rs](https://github.com/bohdaq/file-ext/blob/main/src/lib.rs)

## Crate
[Link to crate release](https://crates.io/crates/file-ext).

## Build
If you want to build `file-ext` on your own, make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

> $ cargo build


## Test
If you want to test `file-ext`.

> $ cargo test


## Community
Use GitHub [discussions](https://github.com/bohdaq/file-ext/discussions), [issues](https://github.com/bohdaq/file-ext/issues) and [pull requests](https://github.com/bohdaq/file-ext/pulls).

There is Rust Web Server [Discord](https://discord.gg/zaErjtr5Dm) where you can ask questions and share ideas.

Follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Donations
Send me tips via [PayPal](https://www.paypal.com/donate/?hosted_button_id=7J69SYZWSP6HJ).

## Links
1. [Rust Web Server](https://github.com/bohdaq/rust-web-server)
1. [http-to-https-letsencrypt](https://github.com/bohdaq/rust-http-to-https-letsencrypt-acme)
1. [Rust Web Framework](https://github.com/bohdaq/rust-web-framework/)
1. [Create Debian Package](https://github.com/bohdaq/rws-create-deb)
1. [Create RPM Package](https://github.com/bohdaq/rws-rpm-builder)
1. [Homebrew Formula](https://github.com/bohdaq/homebrew-rust-tls-server)