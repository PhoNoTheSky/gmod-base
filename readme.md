# Getting started
### Cloning repo
Clone this repo with ```--recursive``` argument.
### Add to cargo.toml
Set ```../gmod-base``` to the folder where you have cloned this repo.
```
[lib]
crate-type = ["cdylib"]

[dependencies]
gmod-base = { path = "../gmod-base" }
```
### Add to lib.rs
```rust
use gmod_base::prelude::*;

unsafe extern "C" fn test_func(_: *mut lua_State) -> i32 {
    println!("test_func called with args: {}", LUA.get_top());
    0
}

#[no_mangle]
pub unsafe extern "C" fn gmod13_open(state: *mut lua_State) -> i32 {
    LUA = LuaWrapper { state };
    println!("Hello from rust!");

    LUA.push_cfunction(Some(test_func));
    LUA.set_global("test_func");

    0
}

#[no_mangle]
pub unsafe extern "C" fn gmod13_close(_: *mut lua_State) -> i32 {
    0
}
```
### Building module
Run these commands, first builds x64 while second builds x86
```
cargo build --release
cargo build --release --target i686-pc-windows-msvc
```

If the second command shows  ```note: the `i686-pc-windows-msvc` target may not be installed```
, run ```rustup target add i686-pc-windows-msvc```

### Add module to gmod
* x64 build is located in the target/release folder
* x86 build is located in the target/i686-pc-windows-msvc folder

### Supported os
* Windows x86 
* Windows x64

### Supported garry's mod versions
* x86-64 branch

### Build command errors
Garry's Mod needs to be located in same drive as steam has been installed in, installing in other folders will cause issues.
