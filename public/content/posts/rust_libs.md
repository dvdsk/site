---
title: "Crates to build on"
date: 2021-07-05T19:11:15+02:00
draft: false
---

Rust has a slim standard library. It provides many core types such as vectors and strings, abstractions the ecosystem builts on etc. However you will not find random numbers, time and date or other work. Such a small library keeps the ecosystem agile. Anything you might miss in the standard library has long been filled in by the community. Even better there is are many choices. As we figure out how to bend Rusts relativly unique semantics to our will better solutions to standard problems arrive. 

However this does mean you will often have to find a suitable libary. After a while you find you have collected your own standard library. This is mine... for now:

***

#### Errors
If writing a library it is customary for your errors to be types that can be matched on. You do this by representing your errors as varients of an `enum`. The crate _thiserror_ makes it easy to make the enum conveniant. In an application seeing the chain of events that lead to the error helps a great deal. I use `eyre` to build these chains and get beutiful error reports.

- for libraries: [thiserror](https://crates.io/crates/thiserror)
- for applications: [color-eyre](https://crates.io/crates/color-eyre) most documentation lives [here](https://docs.rs/eyre/latest/eyre)

#### Logging
For now _tracing_ can be a bit of a hassle to set up. Note it also provides structured logging. For a simple logging solution grap _log_ and _log4rs_.

- simple applications: [log](https://crates.io/crates/log) + [log4rs](https://crates.io/crates/log4rs)
- complex systems: [tracing](https://crates.io/crates/tracing)

#### Command Line
- Argument parsers: [clap](https://crates.io/crates/clap)

#### Runtime
- Async runtime: [tokio](https://crates.io/crates/tokio)
- Macros and functions making async easier: [tokio](https://crates.io/crates/futures)

#### Web
- Http client: [reqwest](https://crates.io/crates/reqwest) (async, optionally blocking)
- Http server: [actix-web](https://crates.io/crates/actix-web) (async)
- HTML templating: [yarte](https://crates.io/crates/yarte)
- Remote procedure calls: [tarpc](https://crates.io/crates/tarpc) (async)

#### Data
- High performance embedded database: [sled](https://crates.io/crates/sled)
- Concurrent hashmap: [dashmap](https://crates.io/crates/dashmap)
- Serialization and deserialization to many formats: [serde](https://crates.io/crates/serde).
	You use serde together with a crate that specifies a format to serialize to/from any rust type. See this [list](https://docs.serde.rs/serde) of the formats.

#### Gui
- Cross platform framework: [iced](https://crates.io/crates/iced)

#### Other
- Time and date: [time](https://crates.io/crates/time)
- Random numbers and other randomness: [rand](https://crates.io/crates/rand)
- Regular expressions: [regex](https://crates.io/crates/regex)

#### Standard lib expansion
- Bounded queue: [crossbeam-channel](https://crates.io/crates/crossbeam-channel)

#### Embedded
- Async runtime: [embassy](https://github.com/embassy-rs/embassy)
- Efficient logging: [defmt](https://crates.io/crates/defmt)
- Hardware abstraction: [embedded-hal](https://crates.io/crates/embedded-hal)
- Global variables: [lazy\_static](https://crates.io/crates/lazy_static)
