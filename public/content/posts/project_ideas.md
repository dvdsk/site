---
title: "Project Ideas"
date: 2021-07-05T19:11:15+02:00
draft: false
---

A list of projects that I would like to do but don't have the time for right now.

### Motd dashboard
I really like the look and modularity of [yboetz/motd](https://github.com/yboetz/motd) however it is not fast enough to run on minimal hardware like a pi3. There is [rust-motd](https://github.com/rust-motd/rust-motd) which solves the performance issues by updating a string via a cron job however that misses some features and some modularity. It would be nice to have the best of both worlds.

 #### Idea:
 - Many binaries in a `cargo workspace` each providing some functionality
 - Binaries set up a cron/systemd timer on first run to update cache
 - To set up simply place them in `/etc/update-motd.d`
 - Provide interface to web dashboard with faster refresh

### Playtime Limited Minecraft server
Usually a minecraft world has a few players that far exceed the median playtime. They build/achieve a lot more then the other players. This can be demotivating for other players. A wrapper around the minecraft server could monitor playtime and limit it to somewhere around the median playtime. This could even motivate players that want more playtime to keep the server active.

### Mpd statistics wrapper
Mpd wrapper to keep statistics on most played/least played often skipped numbers etc. Can generate report listing numbers that should probably be deleted. Maybe even a CLI interface to run through those numbers interactively (listing then deleting or keeping).
