---
title: "Testing needs unsafe"
date: 2023-04-18
draft: true
---

Sometimes I am just coding around something for myself and I forget all about *red-green-refactor* and TDD. And then that little snippet of code gets larger, and suddenly it's an app or a project. Even worse I might start relying on this now out of control snippet of code. And then the tests need to be bolted on. Usually it's a bit of a chore but okay, this time however things got spicy.

About a year back I could not get *certbot* (a python CLI to request and renew TLS certs) working on my old pi. As a proud rustacean 🦀 I build a tiny certificate renewal CLI. Scarred by all dependency hell certbot left me in this new cli had zero runtime dependencies, not even libc (thanks musl!). Whenever I stumble renewing a cert I like to make it impossible to repeat the stumble. One year on [renewc](davidsk.dev/renewc) has become quite a useful tool.

That means adding tests! Today we are building a test for an advanced diagnostic in [renewc](davidsk.dev/renewc). Whenever we renew a certificate *Lets encrypt* needs to verify we are indeed controlling the certificates' domain. They do that by sending us a secret. Then they request the secret by sending a msg to the domain on port 80. Need admin rights to listen on ports below 1025. If they get the secret then we must be control the domain. 

Thus if It helps out when haproxy listening on port 80 and figures out what port that forwards too.
