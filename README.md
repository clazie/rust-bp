# Rust Boilerplate
A boilerplate for a new project in Rust.

## Install Rust and Cargo

The easiest way to get Cargo is to install the current stable release of Rust by using rustup. Installing Rust using rustup will also install cargo.

On Linux and macOS systems, this is done as follows:

``` bash
curl https://sh.rustup.rs -sSf | sh
```
It will download a script, and start the installation. If everything goes well, you’ll see this appear:

Rust is installed now. Great!

On Windows, download and run [rustup-init.exe](https://win.rustup.rs/). It will start the installation in a console and present the above message on success.

After this, you can use the rustup command to also install beta or nightly channels for Rust and Cargo.

For other installation options and information, visit the [install](https://www.rust-lang.org/tools/install) page of the Rust website.


# Build and run
To start run
```
cargo build
cargo run
```