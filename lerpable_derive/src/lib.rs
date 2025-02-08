[package]
name = "lerpable-derive"
version = "0.0.1"
edition = "2021"
authors = ["Jessica Stringham <jessica@thisxorthat.art>"]
repository = "https://github.com/jessstringham/lerpable.git"
license = "MIT"
description = "a derive macro for Lerpable"

[lib]
proc-macro = true

[dependencies]
syn = "2.0.15"
quote = "1.0.18"
proc-macro2 = "1.0.37"
darling = "0.20.3"

lerpable = { version = "0.0.1", path = "../lerpable"}
