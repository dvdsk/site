---
title: "Unsafe testing"
date: 2023-04-18
draft: true
---

Sometimes I am just coding around something for myself and I forget all about *red-green-refactor* and TDD. And then that little snippet of code gets larger, and suddenly it's an app or a project. Even worse I might start relying on this now out of control snippet of code. And then the tests need to be bolted on. Usually it's a bit of a chore but okay, this time however things got spicy.

About a year back I could not get *certbot* (a python CLI to request and renew TLS certs) working on my old pi. As a proud rustacean 🦀 I build a tiny certificate renewal CLI. Scarred by all dependency hell certbot left me in this new CLI had zero runtime dependencies, not even libc (thanks musl!). Whenever I stumble renewing a cert I like to make it impossible to repeat the stumble. One year on [renewc](davidsk.dev/renewc) has become quite a useful tool. That means adding tests! 

Whenever we request a certificate for a domain *Lets encrypt* needs to verify we are in control of it. They do that by sending us a secret. Then they request the secret by sending a msg to the domain on port 80. Need admin rights to listen on ports below 1025. If they get the secret then we must be control the domain. [^1] 

The cool thing about [renewc](davidsk.dev/renewc) is that if things go wrong it tries to help out. For example if you have HAProxy listening on port 80 forwarding traffic to another port. Now let's write an integration test for that. We move most of the CLI's main function to a library. Then we add a test file `tests/diagnostics.rs` with:

```rust
#[test]
fn haproxy_binds_port() {
    let fake_haproxy = spawn_fake_haproxy();
    let mut config = Config::for_test();
    config.port = fake_haproxy.port;

    let err = renewc::run(config).unwrap_err();
    assert!(err.contains("haproxy is forwarding"))
}
```

We start a fake HAProxy that binds to any free port. Then we set the config to the port fake HAProxy is using. It will now fail when it tries to host a secret on that port. Finally, we check if the error contains the hint about HAProxy forwarding.

Let's run it! This fails... Let's print the error, we get: 

```bash
Error:
   0: Challenge server ran into problem
   1: The port is already in use
   2: error creating server listener: Address in use (os error 98)
   3: Address in use (os error 98)

Note: The port is being used by:
	- `diagnostic-f27`
```

It figured out the port was already used but not by anyone called "haproxy". Something may be wrong with our `fake_haproxy`. 

```rust
pub fn spawn_fake_haproxy() -> FakeHAProxy {
    let binder = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = binder.local_addr().unwrap().port();
    println!("fake haproxy listening on: {port}");
    let handle = thread::Builder::new().name("haproxy".into()).spawn(move || {
        for _ in binder.incoming() {}
    });
    FakeHAProxy { handle, port }
}
```

The thread with name `haproxy` gets recognized by our code as `diagnostics-f27`... Now I was not already using this in 'production' therefore the bug was probably in the test. I will be honest here I wasted some time doubting `Builder::name` trying the crate[^2] `proctitle` to set the name. Let's skip over that here. 

Let's take a closer look at `spawn_fake_haproxy`. We add an endless loop before the tests `assert`, that gives us time to actually see what is going on. I used `netstat` however we are not supposed to anymore as its deprecated. Instead, we will use `ss` part of the `iproute` tools. Those interface with the kernel in the same way as [renewc's](davidsk.dev/renewc) does through the `netstat2-rs` crate. 

```bash
ss -ap | grep 42511
tcp LISTEN 0 128 127.0.0.1:42511 0.0.0.0:* users:(("diagnostics-f27",pid=55102,fd=6))
```

We let `ss` know we want to see all sockets and to show the processes using them. Then we filter out any sockets not using the port fake HAProxy is listening on (42511). 

Well `renewc` isn't wrong the OS also sees the process as diagnostics-f27[^3]. Taking a look with process status or as I know it `ps aux`. 

```bash
USER  PID   .... COMMAND
david 55102 .... /home/david/Documents/renewc/target/debug/deps/diagnostics-f27a04fdd8adb5f0 haproxy_binds_port
```

Strange, so here it gets the name `haproxy_binds_port` where `ss` seems to be showing the first 15 characters of the test binaries name. We ran the test using `cargo test haproxy_binds_port`, that explains the name process status is showing. As a spend some time off page setting the process name manually I know about `PR_SET_NAME` from the `prctl` man pages. There we learn that names longer then 15 bytes are truncated. 

On Linux the OS exposes a ton of info to us via the file system. Process related info lives in `/proc/<PID>`. Let's explore around!
```bash
cat /proc/55102/comm                                                                   ✔
diagnostics-f27
cat /proc/55102/cmdline
/home/david/Documents/renewc/target/debug/deps/diagnostics-f27a04fdd8adb5f0haproxy_binds_port
```

So the command line contains the full binary name and its arguments whereas comm has the process name. Neither mentions HAProxy, so where does our thread name come into play?

```bash
ls /proc/$PID/task
55102  55103  55104
```

More process identifiers! These tasks must be the threads: the first is the test harness, the second our test function and finally the named thread it spawned. Is 164682 named "haproxy"?

```bash
cat /proc/164680/task/164682/comm                                                       ✔
haproxy
```

Yes it is! So naming the thread is working but `renewc` isn't working with thread names but with process names?



[^1]: This is the simplest method known as `http-01` there are two others `dns-01` and `tls-alpn-01`.
[^2]: In rust external libraries are called crates.
