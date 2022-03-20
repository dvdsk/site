---
title: "About"
date: 2022-03-20T15:04:16+01:00
draft: true
menu: "main"
---

Summary
=======

I am a software engineer experience in building async services. I work
with a focus on maintainability. I am an active proponent of
refactoring. I have deployed to Linux hosts and have worked with tools
like Docker, SystemD and HaProxy. My personal projects include a service
to on demand move a Minecraft server instance between systems as well as
several open source projects. Serving as chair of the Faculty Council I
have led meetings and helped shape University policy.

Education
=========
- 2019--2022 **Msc Computer Science** _Leiden University_ **Cum Laude**   
_Focused on cloud and distributed computing, explored other fields including AI, robotics and CS-theory_
- 2014--2019 **Double Bsc Physics and Astronomy** _Leiden University_


Skills
======

### Software Engineering

My main objective when developing is to produce readable code that is
easy to change, which I can then more easily turn into high performance
code where needed. I have adopted **test driven development** as it
makes refactoring less risky, meaning it is much easier to keep a clean
and readable code-base. Building a **distributed** file system I learned
behavior can change radically with load and the importance of testing
under load.

### Programming Languages

-   **Rust**: I love writing Rust code. I believe it has made me a
    better programmer, I have become more aware of object and reference
    **lifetimes** and **concurrency** safety when using other languages.

-   **C++**: has major issues regarding memory safety. I try to mitigate
    this by using the standard library in debug mode, using **address
    sanitizer** and encapsulating resources where possible.

-   **Python**: I am a strong proponent of type hinting in Python as it
    helps linters detect more errors before run-time. I use Python for a
    wide variety of tasks ranging from data science to building
    **scaling cloud** systems.

-   **Bash**: Because of its ubiquity I have become quite proficient at
    using Bash to automate DevOps tasks, such deployment and interactive
    monitoring of a cluster during testing. I would not, however, use
    the error-prone arcane wizardry of Bash outside of development and
    testing.

-   **Lua**: I have used Lua to script a large **Neovim** plugin. I
    appreciate its performance, though found it is a challenge to
    structure code well.

### DevOps

-   **Linux**: I have used Linux on my desktop and servers since 2014.
    After trying Fedora and Debian, I settled on using Ubuntu as my
    daily driver, because it offers stability and LTS as well as
    up-to-date packages.

-   **Sandboxing**: My services are sandboxed with **SystemD** instead
    of container engines such as **Docker** and **Podman**. I use these
    to simplify setting up reproducible builds and run-time
    environments.

-   **Architecture** I use a **micro-services** architecture for some of
    my larger projects . I use **HaProxy** for both HTTP and TCP TLS
    termination and routing.


Open Source
-----------

- [Prosesitter](https://github.com/dvdsk/prosesitter.nvim) (Lua),  
_A Neovim extension performing grammar, spelling and style checker for strings and comments in any programming language using ASTs_
- [WorldSync](https://github.com/dvdsk/WorldSync) (Rust),   
_A gui client and server binary to host a Minecraft Server on demand on users machines and synchronize the world_
- [Dataserver](https://github.com/dvdsk/dataserver) (Rust),   
_A compact time series data backend optimized for linear searches_

I maintain other side projects including cross platform [podcast player](https://github.com/dvdsk/pods) and [alarm](https://github.com/dvdsk/alarm) apps and a [home automation system](https://github.com/dvdsk/HomeAutomation) which has been in use, expanded and **maintained** for over 5 years. I also dabble in **async embedded** for example I build a basic [oscilloscope](https://github.com/dvdsk/rustyscopes) to debug a project for a robotics course. I have contributed code to various projects on **GitHub**.

Other Experiances
-----

I served one year as **Chair of the Faculty Council** leading the
council meetings and successfully negotiating to keep a number of
courses available to all students. I really value my years of giving
demonstrations and **tours to the public** at the old observatory in
Leiden and **training** groups of aspirant tour guides.
