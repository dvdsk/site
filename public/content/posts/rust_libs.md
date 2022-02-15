---
title: "Crates to build on"
date: 2021-07-05T19:11:15+02:00
draft: false
---

Rust has a slim standard library. It provides many core types such as vectors and strings, abstractions the ecosystem builds on. However, you will not find random numbers nor time and date in the standard library. Luckily anything you might miss has long been filled in by the community. Sporadically innovative new crates (third party libraries) pop up for problems usually solved in the standard library. The slimness removes the standard library as competitor for these crates, lowering the bar to the spotlight. There is a downside, we can't just take the standard libraries solution but need to figure out crate provides what we need. 

Here are the crates I rely on:

***

#### Errors
If writing a library it is customary for your errors to be types that can be matched on. You do this by representing your errors as variants of an `enum`. The crate _thiserror_ makes it easy to make the enum convenient. In an application seeing the chain of events that lead to the error helps a great deal. I use `eyre` to build these chains and get beautiful error reports.

- for libraries: [thiserror](https://crates.io/crates/thiserror)
- for applications: [color-eyre](https://crates.io/crates/color-eyre) most documentation lives [here](https://docs.rs/eyre/latest/eyre)

#### Logging
You are probably familiar with a backtrace, often shown as a list of functions your program entered to get to the point where it crashed. Tracing gives you on demand 'backtraces' without crashing. You can even have it store parameters. Traces can be inspected by hand (from a file log for example) or using tools such as [jeager](https://www.jaegertracing.io). Right now _tracing_ can still be a bit of a hassle to set up. For simple applications you might want to use _log_ with _log4rs_.

- simple applications: [log](https://crates.io/crates/log) + [log4rs](https://crates.io/crates/log4rs)
- complex systems: [tracing](https://crates.io/crates/tracing) also provides (structured) logging

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
