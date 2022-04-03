---
title: "Projects"
date: 2022-03-17T16:11:15+02:00
draft: false
aliases:
    - /p
---

Over the past years I have worked on a lot of side projects, using software to solve some perceived problem. This is a list of my side work, the motivations behind it and issues a ran into. Most recent projects first.

### [Book-safe](https://github.com/dvdsk/Book-safe)
I kept "forgetting" the time reading the novels by _Robin Hobb_. I spend the better part of 3 weekends writing a service for my [eink-tablet](https://remarkable.com/) which locks me out of her books past bed time. As any update might change how the device works, I had to figure out a method of hiding items from the UI, so that it could not lose documents or brick the device. 

The service adds a report document what it hid and until when. For each document on the tablet there is a JSON metadata file. I had to generate it for the report. It took me way too long to notice I had missed a pair of braces. I now know better than to format JSON manually. 

During development I used a [trait](https://github.com/dvdsk/Book-safe/blob/723683aa100f7c268334d4fdf956b3f881a75719/src/util.rs#L10) to add a neat [function](https://github.com/dvdsk/Book-safe/blob/723683aa100f7c268334d4fdf956b3f881a75719/src/main.rs#L88) to Rust's Result type to simplify error handling a bit more.

### [WorldSync](https://github.com/dvdsk/WorldSync)
When playing Minecraft with a group, either one person in the group hosts, or you rent a VPS. Being a dutchman, and thus thrifty, the latter was of course no option. _WorldSync_ is something in between. Group members start _WorldSync_ and join the server that is started in the background on their machine, or ran by a member already playing. Coordination and synchronization is done using a central server running on an old raspberry pi. For _WorldSync_ to be a good alternative to a VPS, sync must be transparent to the user. The size of the saves (>GB) presented a problem. So to speed up the sync saves are split into objects that are stored deduplicated on the central server. The server only needs to send changes and incremental back-upping became easy and costs little space.

_WorldSync_ is mostly done, save for some bug-fixing which will happen when we start playing Minecraft again.

### [Raft-fs](https://github.com/dvdsk/raft-fs)
An experimental distributed file-system I build for a class in distributed computing. It tries to improve on the Google FS architecture by using multiple metadata servers. The original architecture had only a single metadata node which formed a single point of failure. The additional meta data nodes can also increase performance for metadata reads. I used raft to maintain consistency between a primary and multiple read-only metadata servers. Performance under load was disappointing, as heartbeats between nodes started timing out. My master thesis builds upon this idea scaling both read and write performance and solving the heart beat timeouts.

### [prosesitter.nivm](https://github.com/dvdsk/prosesitter.nvim)
A work in progress _neovim_ plugin trying to offer spell, grammar and text linting for all programming languages. It uses treesitter to extract human readable text (prose) which it forwards to _vale_ and _languagetool_. The output of those tools is used to underline text and provide hints describing the error together with a fix that can automatically be applied. The backends are called asynchronously and only on changed prose, resulting in little to any performance impact.

When I started development I chose not to do test driven development. It seemd hard to set up, as _prosesitter_ heavily integrates with neovim's api. This was a big mistake. Refactoring was terrifying. Every bug I crushed seems to create one in the support of another language. I spend a few hours each week refactoring and fixing bugs (expanding the test suite for every bug found). Today I have a good set of integration tests and not enough unit tests.

A challenge for me personally was the simplicity of _Lua_. It features tables, strings, numbers, booleans, functions and that's it. Without strong types and having few to any data-structures it was hard to create the needed structure for a project of this size.

Right now Markdown support is almost correct and I hope to finish Latex support before my thesis is done.

### [Pods](https://github.com/dvdsk/pods)
A fast and _very_ basic podcast app that can download and stream with an integrated audio player. It can search podcasts by name and adds new episodes on startup. I created it to fill the void of mobile Linux podcast apps when I got a PinePhone (Linux phone). I still use it for listening and hope to expand it in the future, though I would not recommend it to anyone in its current state. I spent a lot of time back-porting the UI framework to the old OpenGL version the PinePhone uses ([fork](https://github.com/dvdsk/iced)).

### [HomeAutomation](https://github.com/dvdsk/HomeAutomation)
Home automation was my gateway drug into programming. This project started in Python, before being rewritten in C++, and Rust a few years back. Originally it was quite the monolith: it read sensors, stored and visualized data and controlled lighting and music. Today the first two tasks have been split off into: [sensor-central](https://github.com/dvdsk/sensor_central) and [data-server](https://github.com/dvdsk/dataserver). Maintenance is hands off thanks to cert-bot and live reloading with HaProxy.

### Other
There are many more side projects I worked on such as an [alarm app](https://github.com/dvdsk/alarm) for my home automation system, a [simple database](https://github.com/dvdsk/minimal_timeseries) focused on linear lookup. A [compressed integer](https://github.com/dvdsk/bitspec) library. An [experiment](https://github.com/dvdsk/error_level) adding log levels to enum variants and more ...
