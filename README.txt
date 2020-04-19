A work in progress to add a WASM archetecture to Open Computers.
Because some of us like C, C++, and Rust.

Don't expect much documentation until its closer to finished.


Building
Your typical gradle build command is how you fire off the build for this
project, but because the WASM interpreter is implemented in Rust, you will need
to install Rust and its build system Cargo.

You can install both of those from here: https://rustup.rs/

Cross compilation is much easier with Rust than it is with C or C++, but it's
still fairly tricky. If you only wish to compile for your native OS, you can
disable the compilation of certain versions of the native libraries in the
gradle.properties file.

I cross compile from Linux to Windows, and from Linux to Mac OS.

This article was essential in getting the cross compilation to windows working:
https://wiki.archlinux.org/index.php/Rust#Cross_compiling

This artical was essential in getting the cross compilation to Mac OS working:
https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html
You only need to follow this article up to and through the part labeled
"Building the project". I have taken care of the rest for you in the gradle
scripts.