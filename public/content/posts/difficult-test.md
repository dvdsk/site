---
title: "Testing needs unsafe"
date: 2023-04-18
draft: true
---

Sometimes I am just coding around something for myself and I forget all about *red-green-refactor* and TDD. And then that little snippet of code gets larger, and suddenly it's an app or a project. Even worse I might start relying on this now out of control snippet of code. And then the tests need to be bolted on. Usually it's a bit of a chore but okay, this time however things got spicy.

About a year back I could not get *certbot* (a python CLI to request and renew TLS certs) working on my old pi. As a proud rustacean 🦀 I build a tiny certificate renewal CLI. Scarred by all dependency hell certbot left me in this new CLI had zero runtime dependencies, not even libc (thanks musl!). Whenever I stumble renewing a cert I like to make it impossible to repeat the stumble. One year on [renewc](davidsk.dev/renewc) has become quite a useful tool. That means adding tests! 

Whenever we request a certificate for a domain *Lets encrypt* needs to verify we are in control of it. They do that by sending us a secret. Then they request the secret by sending a msg to the domain on port 80. Need admin rights to listen on ports below 1025. If they get the secret then we must be control the domain. [^1] 

The cool thing about [renewc](davidsk.dev/renewc) is that if things go wrong it tries to help out. For example if you have HAProxy listening on port 80 forwarding traffic to another port. Now let's write an integration test for that. We move most of the CLI's main function to a library. Then we add a test file `tests/diagnostics.rs` with:

```rust
#[test]
fn diagnose_haproxy_bind() {
    let fake_haproxy = spawn_fake_haproxy();
    let mut config = Config::for_test();
    config.port = fake_haproxy.port;

    let err = renewc::run(config).unwrap_err();
    assert!(err.contains("haproxy is forwarding"))
}
```

We start a fake HAProxy that binds to any free port. Then we set the config to the port fake HAProxy is using. It will now fail when it tries to host a secret on that port. Finally, we check if the error contains the hint about HAProxy forwarding.

This fails... Let's print the error, we get: 

```bash
Error:
   0: Challenge server ran into problem
   1: The port is already in use
   2: error creating server listener: Address in use (os error 98)
   3: Address in use (os error 98)

Note: The port is being used by:
	- `diagnostic-f6`
```

It figured out the port was already used but not by anyone called "haproxy". Something is wrong with our `fake_haproxy`. 

```rust
pub fn spawn_fake_haproxy() -> FakeHAProxy {
    let binder = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = binder.local_addr().unwrap().port();
    let handle = thread::Builder::new().name("haproxy".into()).spawn(move || {
        for _ in binder.incoming() {}
    });
    FakeHAProxy { handle, port }
}
```

The thread with name `haproxy` gets recognized by our code as `diagnostics-f6`... Now I was already using this in 'production' (this very server) therefore maybe the test was wrong? I will be honest here I wasted some time doubting `Builder::name` trying the crate `proctitle` to set the name. Let's skip over that here.


[^1]: This is the simplest method known as `http-01` there are two others `dns-01` and `tls-alpn-01`.
