---
title: "Projects"
date: 2021-07-05T19:11:15+02:00
draft: true
---

Over the past years I have worked on a lot of side projects, using software to solve some perceived problem. This list of my work, the motivations behind and issues a ran into. Most recent projects first.

### [Book-safe](https://github.com/dvdsk/Book-safe)
I kept "forgetting" the time reading the work of _Robin Hobb_. I spend the better part of 3 weekends writing a service for my [eink-tablet](https://remarkable.com/) which locks my out of her work past bed time. As any update might change how the device works I had to figure out a method of hiding items from the UI that could not lose my notes or brick the device. The service adds a report document what it hid and until when. For each document on the tablet there is a JSON metadata file. It took me way to long to notice the missing braces in the JSON. I now know better then to format JSON manually. During development I used a [trait](https://github.com/dvdsk/Book-safe/blob/723683aa100f7c268334d4fdf956b3f881a75719/src/util.rs#L10) to add a neat [function](https://github.com/dvdsk/Book-safe/blob/723683aa100f7c268334d4fdf956b3f881a75719/src/main.rs#L88) to Rust's Result type to simplefy error handling a bit more.

### [WorldSync](https://github.com/dvdsk/WorldSync)
Me and some friends share a Minecraft server. Usually one person in the group hosts or the group rents a VPS. Being Dutch the latter was of course no option. _WorldSync_ is something in between. It transparently synchronizes server data and starts if no one else is hosting. Group members start _WorldSync_ and join the server either running on their machine or that of another member. Coordination and (data+run-time) synchronization is done using a central server running on an old raspberry pi. For everything synchronization to be transparent to the user it must finish quickly. Minecraft saves are quite large (>GB) however. Saves are split into objects that are stored deduplicated on the central server. This way the server only needs to send changes and incremental back-upping became easy. 

_WorldSync_ is mostly done save some bug-fixing which will happen when we start playing Minecraft again.

### [Raft-fs](https://github.com/dvdsk/raft-fs)
An experimental distributed file-system I build for a class in distributed computing. It tries to improve on the Google FS architecture by using multiple metadata servers. The original architecture had only a single metadata node which formed a single point of failure. The additional meta data nodes can also increase performance for metadata reads. I used raft to maintain consistency between a primary and multiple read-only metadata servers. Performance under load was disappointing as heartbeats between nodes started timing out. My master thesis builds upon this idea scaling both read and write performance and solving the heart beat timeouts.

### [prosesitter.nivm](https://github.com/dvdsk/prosesitter.nvim)
A work in progress _neovim_ plugin trying to offer spell, grammar and text linting for all programming languages. It uses treesitter to extract human readable text forwards it to _vale_ and _languagetool_. The output of these tools is used to underline text and provide a popup describing the error and a suggestion that can automatically be applied. The backends are called asynchronously and only on changed text resulting in a unnoticeable performance impact. 

When I started I chose not to use test driven development. It seems really hard to set up as _prosesitter_ heavily integrates with neovim's api. It was a big mistake. Refactoring was terrifying. Every bug I crushed seems to create one in the support of another language. Today I have a good set of integration tests and not enough unit tests. I spend a few hours each week refactoring and fixing bugs, expanding the test suite for every bug found. Markdown support is almost working correctly and I hope to finish Latex support before my thesis is done.

### [Pods](https://github.com/dvdsk/pods)
A lite and _very_ basic podcast app that can download and stream with an integrated audio player. It can search podcasts by name and adds new episodes on startup. It was created to fill the void of mobile Linux podcast apps. I still use it for listening and hope to expand it in the future. I spend a lot of time back-porting the UI framework to the old OpenGl version the PinePhone uses ([fork](https://github.com/dvdsk/iced)).

### [HomeAutomation](https://github.com/dvdsk/HomeAutomation)
This was my gateway drug into programming. It started in Python before being rewriting to C++ and a few years back Rust. Originally it was quite the monolith. It read sensors, stored and visualized data and controlled lighting and music. Today the first two tasks have been split off into: [sensor-central](https://github.com/dvdsk/sensor_central) and [data-server](https://github.com/dvdsk/dataserver). Maintenance is quite hands off terminating TLS with HaProxy (useful for live reloading) and certbot for automatic certificate renewal.

### Other
There are many more side projects I worked on such as an [alarm](https://github.com/dvdsk/alarm) app for my home automation system, a simple database focused on linear lookup [minimal_timeseries](https://github.com/dvdsk/minimal_timeseries). A compressed integer library [bitspec](https://github.com/dvdsk/bitspec). An experiment adding log levels to enum variants [error_level](https://github.com/dvdsk/error_level) and more ...
