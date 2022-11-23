---
title: "Crates to build on"
date: 2021-12-05T12:18:15+01:00
draft: false
aliases:
    - /build_on
---

Rust has a slim standard library. It provides many core types such as vectors and strings, abstractions the ecosystem builds on. You will not, however find random numbers or time and date in the standard library. Luckily anything you might miss has long been filled in by the community. Innovative new crates (third party libraries) also pop up for problems usually solved in the standard library. The slimness removes the standard library as competitor for these crates, lowering the bar to the spotlight. The downside is that we can't just take the standard libraries' solution but need to figure out, which crate provides what we need. 

Here are the crates I rely on:

***

#### Errors
In Rust its customary for your errors to be types that allow pattern matching when writing a library. Normally you accomplish by representing your errors as variants of an `enum`. The crate _thiserror_ helps you to make the enum convenient to use. When writing an application however, you match on errors, handling them or reporting them. I use `eyre` to build beautiful error reports that give a useful backtrace.

- For libraries: [thiserror](https://crates.io/crates/thiserror)
- For applications: [color-eyre](https://crates.io/crates/color-eyre) most documentation lives [here](https://docs.rs/eyre/latest/eyre)

- To check/prove a function can't crash your program: [no-panic](https://crates.io/crates/no-panic)

#### Logging
You are probably familiar with backtraces, often shown as a list of functions your program entered to get to a point where it crashed. Tracing gives you on demand 'backtraces' without crashing. It can include the parameters for all the called functions. The traces can be inspected by hand (from a log file) or using tools such as [jeager](https://www.jaegertracing.io). Right now _tracing_ can still be a bit of a hassle to set up. For simple applications you might want to use _log_ with _simplelog_.

- Simple applications: [log](https://crates.io/crates/log) + [simplelog](https://crates.io/crates/simplelog) \
  hint: use [add_filter_ignore](https://docs.rs/simplelog/latest/simplelog/struct.ConfigBuilder.html#method.add_filter_ignore) to stop verbose dependencies from flooding the log _(useful at low log levels)._
- Complex systems: [tracing](https://crates.io/crates/tracing) also provides (structured) logging

#### Tests
- checking float equality: [float_eq](https://crates.io/crates/float_eq) \
	basic usage: `assert_float_eq!(a, b, ulps <= 1)` for more see: [docs](https://jtempest.github.io/float_eq-rs/book/tutorials/basic_usage.html)

#### Command Line
- Argument parsers: [clap](https://crates.io/crates/clap)

#### Runtime
- Async runtime: [tokio](https://crates.io/crates/tokio)
- Macros and functions making async easier: [futures](https://crates.io/crates/futures)

#### Web
- Http client: [reqwest](https://crates.io/crates/reqwest) (async, optionally blocking)
- Http server: [actix-web](https://crates.io/crates/actix-web) (async)
- HTML templating: [yarte](https://crates.io/crates/yarte)
- Remote procedure calls: [tarpc](https://crates.io/crates/tarpc) (async)

#### Data
- High performance embedded database: [sled](https://crates.io/crates/sled)
- Concurrent hashmap: [dashmap](https://crates.io/crates/dashmap)
- Lock free eventually consistent map [evmap](https://crates.io/crates/evmap)
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
- Derive traits: [derivative](https://crates.io/crates/derivative)

#### Embedded
- Async runtime: [embassy](https://github.com/embassy-rs/embassy)
You will need to use a git dependency as _embassy_ is still early in development. Do not forget to add a `rev` key with a specific commit hash to the `Cargo.toml` entry to ensure reproducable builds.
- Efficient logging: [defmt](https://crates.io/crates/defmt)
- Hardware abstraction: [embedded-hal](https://crates.io/crates/embedded-hal)
- Global variables: [lazy\_static](https://crates.io/crates/lazy_static)
