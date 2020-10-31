# Buttplug (Rust Implementation)

[![Patreon donate button](https://img.shields.io/badge/patreon-donate-yellow.svg)](https://www.patreon.com/qdot)
[![Github donate button](https://img.shields.io/badge/github-donate-ff69b4.svg)](https://www.github.com/sponsors/qdot)
[![Discourse Forum](https://img.shields.io/badge/discourse-forum-blue.svg)](https://metafetish.club)
[![Discord](https://img.shields.io/discord/353303527587708932.svg?logo=discord)](https://discord.buttplug.io)
[![Twitter](https://img.shields.io/twitter/follow/buttplugio.svg?style=social&logo=twitter)](https://twitter.com/buttplugio)

[![Crates.io Version](https://img.shields.io/crates/v/buttplug)](https://crates.io/crates/buttplug)
[![Crates.io Downloads](https://img.shields.io/crates/d/buttplug)](https://crates.io/crates/buttplug)
[![Crates.io License](https://img.shields.io/crates/l/buttplug)](https://crates.io/crates/buttplug)

<div align="center">
  <h3>
    <a href="https://docs.rs/buttplug">
      API Documentation
    </a>
    <span> | </span>
    <a href="https://buttplug-spec.docs.buttplug.io">
      Protocol Spec
    </a>
    <span> | </span>
    <a href="https://buttplug-developer-guide.docs.buttplug.io">
      Developer Guide
    </a>
    <span> | </span>
    <a href="https://github.com/buttplugio/buttplug-rs/releases">
      Releases
    </a>
  </h3>
</div>

<p align="center">
  <img src="https://raw.githubusercontent.com/buttplugio/buttplug-rs/dev/buttplug/docs/buttplug_rust_docs.png">
</p>

Rust implementation of the Buttplug Intimate Hardware Protocol,
including implementations of the client and, at some point, server.

This repo is a monorepo with 2 projects:

- [buttplug](buttplug/) - Main library
- [buttplug_derive](buttplug_derive/) - Procedural macros used by the buttplug rust library.

For information about compiling and using these libraries, please check the
README files in their directories.

## Read Me First!

If you are new to Buttplug, you most likely want to start with the
[Buttplug Website](https://buttplug.io) or the [Buttplug Core
Repo](https://github.com/buttplugio/buttplug).

For a demo of what this framework can do, [check out this demo
video](https://www.youtube.com/watch?v=RXD76g5fias).

Buttplug-rs is a full fledged implementation of Buttplug, on par with
our [C#](https://github.com/buttplugio/buttplug-csharp) and
[Javascript/Typescript](https://github.com/buttplugio/buttplug-js)
implementations.

Buttplug-rs is currently capable of controlling toys via:

- Bluetooth LE
- Serial Ports
- USB HID
- Lovense Devices via the Lovense Dongle (All Versions)
- XInput gamepads (Windows only)

Note that only some protocols/hardware is currently supported. See
[IOSTIndex](https://iostindex.com) for more info.

## Introduction

[Buttplug](https://buttplug.io) is a framework for hooking up hardware
to interfaces, where hardware usually means sex toys, but could
honestly be just about anything. It's basically a userland HID manager
for things that may not specifically be HID.

In more concrete terms, think of Buttplug as something like
[osculator](http://www.osculator.net/) or [VRPN](http://vrpn.org), but
for sex toys. Instead of wiimotes and control surfaces, we interface
with vibrators, electrostim equipment, fucking machines, and other
hardware that can communicate with computers.

The core of buttplug works as a router. It is a Rust based application
that connects to libraries that registers and communicates with
different hardware. Clients can then connect over websockets or
network ports, to claim and interact with the hardware.

## Contributing

Right now, we mostly need code/API style reviews and feedback. We
don't really have any good bite-sized chunks to mentor the
implementation yet, but one we do, those will be marked "Help Wanted"
in our [github
issues](https://github.com/buttplugio/buttplug-rs/issues).

As we need money to keep up with supporting the latest and greatest hardware, we
also have multiple ways to donate!

- [Patreon](https://patreon.com/qdot)
- [Github Sponsors](https://github.com/sponsors/qdot)
- [Ko-Fi](https://ko-fi.com/qdot76367)

## License

Buttplug is BSD licensed.

    Copyright (c) 2016-2019, Nonpolynomial Labs, LLC
    All rights reserved.

    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this
      list of conditions and the following disclaimer.

    * Redistributions in binary form must reproduce the above copyright notice,
      this list of conditions and the following disclaimer in the documentation
      and/or other materials provided with the distribution.

    * Neither the name of buttplug nor the names of its
      contributors may be used to endorse or promote products derived from
      this software without specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
    OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
