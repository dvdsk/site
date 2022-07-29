---
title: "About me"
seoTitle: "David Kleingeld"
draft: false
menu: "main"
---Summary
=======
I am a software engineer experienced in building async services. I work with a focus on maintainability. I have deployed to Linux hosts and have worked with tools like Docker, SystemD and HaProxy. My open source projects include a distributed file system and a macro to turn a struct into a typed database. Serving as chair of the Faculty Council, I have led meetings and helped shape University policy. I enjoy teaching my programming group to write beautiful code.

Education
=======
- 2019--2022 **MSc Computer Science** _Leiden University_ **Cum Laude**  
Focused on cloud and distributed computing, explored other fields including AI, robotics and CS-theory   
 *Thesis: [GovFs](https://davidsk.dev/thesis), a highly scalable consistent distributed file system*
- 2014--2019 **Double BSc Physics and Astronomy** _Leiden University_   


Skills
=======
### Software Engineering

While developing I aim to build readable code that is easy to change. I can then more easily turn into high performance code where needed. I have adopted **test driven development** as it makes refactoring less risky, meaning it is much easier (and realistic) to maintain a clean and readable code-base. Building a **distributed** file system I experienced the importance of running test under high load. To keep my software engineering skills growing I engage with large open source projects, read great titles such as "The Pragmatic Programmer" and "Refactoring", and explore these ideas in side projects.

### Programming Languages

- **Rust**: I love writing Rust code. I believe it has made me a better programmer, I have become more aware of object and reference **lifetimes** and how to use **concurrency** safely when using other languages. 

- **C++**: has major issues regarding memory safety. I try to mitigate this by using the standard library in debug mode, using **address sanitizer** and encapsulating resources where possible. 

- **Python**: I am a strong proponent of type hinting in Python as it helps linters detect more errors before run-time. I use Python for a wide variety of tasks ranging from data science to building **scaling cloud** systems.

- **Bash/Unix CLI**: Because of there ubiquity I have become quite proficient at using Bash and CLI tools to automate **DevOps** tasks, such as: deployment, monitoring a cluster and filtering megabytes of logs. I would not, however, use the error-prone arcane wizardry of Bash outside of development and testing.

- **Lua**: I have used Lua to script a large **Neovim** plugin. I appreciate its performance, though found it is a challenge to structure code well.



### DevOps

- **Linux**: I have used Linux on my desktop and servers since 2014. After trying Fedora and Debian, I settled on using Pop!_OS as my daily driver, because it offers stability and LTS as well as up-to-date packages. 	

- **Sandboxing**: My services are sandboxed with **SystemD** instead of container engines such as **Docker** and **Podman**. I use these to simplify setting up reproducible builds and run-time environments.

- **Architecture**: I use a **micro-services** architecture for some of my larger projects. I use **HaProxy** for both HTTP and TCP TLS termination and routing.



Experience
=======
### Open Source

- [Instance-chart](https://crates.io/crates/instance-chart) (Rust),   
_A typed alternative to mDNS, for service discovery on the same network or machine._
- [dbstruct](https://github.com/dvdsk/dbstruct) (Rust),   
_A macro to turn a struct into a typed database_
- [WorldSync](https://github.com/dvdsk/WorldSync) (Rust),   
_A GUI client and server binary to host a Minecraft Server on demand on users' machines and synchronize the world_
- [Dataserver](https://github.com/dvdsk/dataserver) (Rust),   
_A compact time series data backend optimized for linear searches_



Other projects include: cross-platform [podcast player](https://github.com/dvdsk/pods) and [alarm](https://github.com/dvdsk/alarm) apps, a [home automation system](https://github.com/dvdsk/HomeAutomation) which has been in use, expanded and **maintained** for over 5 years. Furthermore, I dabble in **async embedded** building a basic [oscilloscope](https://github.com/dvdsk/rustyscopes). Finally, I worked on a Neovim extension for grammar, spelling and style checker in strings and comments.  
I have contributed code to various projects on **GitHub** and recently contributed a [proposal](https://github.com/rust-lang/libs-team/issues/55) for expanding the **Rust** standard library.
### Other
I served one year as **Chair of the Faculty Council** leading the council meetings and successfully negotiating to keep a number of courses available to all students.I really value my years of giving demonstrations and **tours to the public** at the old observatory in Leiden and **training** groups of aspirant tour guides.  
Recently I have started a small programming study group where I **teach** Rust and modern extendible Python to computer science students.