Mavio
=====

Minimalistic library for transport-agnostic [MAVLink](https://mavlink.io/en/) communication. It supports `no-std`
(and `no-alloc`) targets.

<span style="font-size:24px">[🇺🇦](https://mavka.gitlab.io/home/a_note_on_the_war_in_ukraine/)</span>
[![
`repository`](https://img.shields.io/gitlab/pipeline-status/mavka/libs/mavio.svg?logo=gitlab&branch=main&label=repository)](https://gitlab.com/mavka/libs/mavio)
[![`mirror`](https://img.shields.io/badge/-gray?logo=github)](https://github.com/istalabs/mavio)
[![`crates.io`](https://img.shields.io/crates/v/mavio.svg)](https://crates.io/crates/mavio)
[![`docs.rs`](https://img.shields.io/docsrs/mavio.svg?label=docs)](https://docs.rs/mavio/latest/mavio/)
[![
`issues`](https://img.shields.io/gitlab/issues/open/mavka/libs/mavio.svg)](https://gitlab.com/mavka/libs/mavio/-/issues/)

<details>
<summary><em>Repositories</em></summary>

> Currently, we use [GitLab](https://gitlab.com/mavka/libs/mavio) as the main project repository and
[GitHub](https://github.com/istalabs/mavio) as official mirror.
>
> We accept [issues](https://gitlab.com/mavka/libs/mavio/-/issues) and
[pull-requests](https://gitlab.com/mavka/libs/mavio/-/merge_requests) only at GitLab but will do our best
> to keep GitHub [discussions](https://github.com/istalabs/mavio/discussions) as alive as possible.
>
> The [mirror](https://github.com/istalabs/mavio) will always contain latest release tags and is kept up to date
> automatically.

</details>

Intro
-----

Mavio is a building block for more sophisticated tools. It is entirely focused on one thing: to include absolute minimum
of functionality required for correct communication with everything that speaks MAVLink protocol.

* Supports both `MAVLink 1` and `MAVLink 2` protocol versions.
* Provides intermediate MAVLink packets decoding as "frames" that contain only header, checksum and signature being
  deserialized. Which means that client don't have to decode the entire message for routing and verification.
* Supports optional high-level message decoding by utilizing MAVLink abstractions generated by
  [MAVSpec](https://gitlab.com/mavka/libs/mavspec).
* Includes standard MAVLink dialects enabled by Cargo features.
* Implements message verification via checksum.
* Includes tools for [message signing](https://mavlink.io/en/guide/message_signing.html).

In other words, Mavio implements all *stateless* features of MAVLink protocol. Which means that it does not provide
support for message sequencing, automatic heartbeats, etc. The client is responsible for implementing these parts of the
protocol by their own or use a dedicated library. We've decided to keep Mavio as simple and catchy as an
[8-bit melody](https://archive.org/details/SuperMarioBros.ThemeMusic).

At the same time, Mavio is flexible and tries to dictate as few as possible. In particular:

* It supports [custom dialects](#custom-dialects) or may work with no dialect at all (for intermediate decoding). The
  latter is useful if you want to simply route or sign messages.
* Can read and write messages to anything that
  implements [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html)
  and [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) traits.
* Compatible with `no_std` targets. For such cases the library provides simplified versions of `Read` and `Write`
  traits.
* Support asynchronous I/O via [Tokio](https://tokio.rs).
* Allows filtering out unnecessary MAVLink entities (i.e. messages, enums, commands) to reduce compilation time.

This library is a part of [Mavka](https://mavka.gitlab.io/home/) toolchain. It is integrated with other projects such
as:

* [MAVInspect](https://gitlab.com/mavka/libs/mavinspect) that responsible for MAVLink XML definitions parsing.
* [MAVSpec](https://gitlab.com/mavka/libs/mavinspect) that focused on code generation. Mavio uses this library to
  generate MAVLink dialects.
* [Maviola](https://gitlab.com/mavka/libs/maviola), is a MAVLink communication library based on `Mavio` that
  provides a high-level interface for MAVLink messaging and takes care about **stateful** features of the protocol:
  sequencing, message time-stamping, automatic heartbeats, simplifies message signing, and so on.

This project respects [`semantic versioning`](https://semver.org). As allowed by specification, breaking changes may be
introduced in minor releases until version `1.0.0` is reached. However, we will keep unstable features under the
`unstable` feature flag whenever possible.

Install
-------

Install Mavio with cargo:

```shell
cargo add mavio
```

Usage
-----

For details, please check [API](#api-notes) section and [API documentation](https://docs.rs/mavio/latest/mavio/).

### Receiving MAVLink Frames

Connect as TCP client, receive 10 frames, and decode any received
[`HEARTBEAT`](https://mavlink.io/en/messages/common.html#HEARTBEAT) message:

```rust
use std::net::TcpStream;

use mavio::{Frame, Receiver};
use mavio::dialects::minimal as dialect;
use mavio::io::StdReader;
use dialect::Minimal;

fn main() -> mavio::errors::Result<()> {
    let mut receiver = Receiver::new(StdReader::new(TcpStream::connect("0.0.0.0:5600")?));

    for i in 0..10 {
        let frame = receiver.recv_frame()?;

        if let Err(err) = frame.validate_checksum::<Minimal>() {
            eprintln!("Invalid checksum: {:?}", err);
            continue;
        }

        if let Ok(Minimal::Heartbeat(msg)) = frame::decode() {
            println!(
                "HEARTBEAT #{}: mavlink_version={:#?}",
                frame.sequence(),
                msg.mavlink_version,
            );
        }
    }

    Ok(())
}
```

A slightly more elaborated use-case can be found in [`tcp_client.rs`](./examples/sync/examples/tcp_client.rs) example.

### Sending MAVLink Frames

Listen to TCP port as a server, send 10 [`HEARTBEAT`](https://mavlink.io/en/messages/common.html#HEARTBEAT) messages
to any connected client using MAVLink 2 protocol, then disconnect a client.

```rust
use std::net::TcpStream;

use mavio::dialects::minimal as dialect;
use mavio::io::StdWriter;
use dialect::enums::{MavAutopilot, MavModeFlag, MavState, MavType};

use mavio::prelude::*;

fn main() -> Result<()> {
    // Create a TCP client sender
    let mut sender = Sender::new(StdWriter::new(TcpStream::connect("0.0.0.0:5600")?));
    // Create an endpoint that represents a MAVLink device speaking `MAVLink 2` protocol
    let endpoint = Endpoint::v2(MavLinkId::new(15, 42));

    // Create a message
    let message = dialect::messages::Heartbeat {
        type_: MavType::FixedWing,
        autopilot: MavAutopilot::Generic,
        base_mode: MavModeFlag::TEST_ENABLED & MavModeFlag::CUSTOM_MODE_ENABLED,
        custom_mode: 0,
        system_status: MavState::Active,
        mavlink_version: 3,
    };
    println!("MESSAGE: {message:?}");

    for i in 0..10 {
        // Build the next frame for this endpoint.
        // All required fields will be populated, including frame sequence counter.
        let frame = endpoint.next_frame(&message)?;

        sender.send_frame(&frame)?;
        println!("FRAME #{} sent: {:#?}", i, frame);
    }

    Ok(())
}
```

Check [`tcp_server.rs`](./examples/sync/examples/tcp_server.rs) for a slightly more elaborated use-case.

API Notes
---------

This section provides a general API overview. For further details, please check
[API documentation](https://docs.rs/mavio/latest/mavio/).

### I/O

Mavio provides two basic I/O primitives: [`Sender`](https://docs.rs/mavio/latest/mavio/struct.Sender.html) /
[`Receiver`](https://docs.rs/mavio/latest/mavio/struct.Sender.html) for synchronous and
[`AsyncSender`](https://docs.rs/mavio/latest/mavio/struct.AsyncSender.html) /
[`AsyncReceiver`](https://docs.rs/mavio/latest/mavio/struct.AsyncSender.html) for asynchronous I/O. These structs send
and receive instances of [`Frame`](https://docs.rs/mavio/latest/mavio/struct.Frame.html).

`Sender` and `Receiver` are generic over [`Write`](https://docs.rs/mavio/latest/mavio/io/trait.Write.html) and
[`std::io::Read`](https://docs.rs/mavio/latest/mavio/io/trait.Read.html) accordingly. While `AsyncSender` and
`AsyncReceiver` use [`AsyncWrite`](https://docs.rs/mavio/latest/mavio/io/trait.AsyncWrite.html) and
[`std::io::AsyncRead`](https://docs.rs/mavio/latest/mavio/io/trait.AsyncRead.html). A set of I/O
[adapters](https://docs.rs/mavio/latest/mavio/io/adapters/index.html) are available for
[std::io](https://doc.rust-lang.org/std/io/), [Tokio](https://tokio.rs), and
[`futures-rs`](https://docs.rs/futures/). That means you can communicate MAVLink messages over various transports
including UDP, TCP, Unix sockets, and files. It is also easy to implement custom transport.

For `no-std` targets Mavio provides adapters for [`embedded HAL`](https://github.com/rust-embedded/embedded-hal) I/O
(both synchronous and asynchronous).

### Encoding/Decoding

Upon receiving, MAVLink [`Frame`](https://docs.rs/mavio/latest/mavio/struct.Frame.html)s can be validated and decoded
into MAVLink messages. Frames can be routed, signed, or forwarded to another system/component ID without decoding.

> **Note!**
>
> MAVLink checksum validation requires [`CRC_EXTRA`](https://mavlink.io/en/guide/serialization.html#crc_extra)
> byte which in its turn depends on a dialect specification. That means, if you are performing dialect-agnostic routing
> from a noisy source or from devices which implement outdated message specifications, you may forward junk messages.
> In case of high-latency channels you might want to enforce compliance with a particular dialect to filter
> incompatible messages.

To decode a frame into a MAVLink message, you need to use a specific dialect. Standard MAVLink dialects are available
under [`mavio::dialects`](https://docs.rs/mavio/latest/mavio/dialects) and can be enabled by the corresponding
feature flags (`dlct-<dialect name>`).

* [`minimal`]((https://mavlink.io/en/messages/minimal.html)) — minimal dialect required to expose your presence to
  other MAVLink devices.
* [`standard`](https://mavlink.io/en/messages/standard.html) — a superset of `minimal` dialect which expected to be
  used by almost all flight stack.
* [`common`](https://mavlink.io/en/messages/common.html) — minimum viable dialect with most of the features, a
  building block for other future-rich dialects.
* [`ardupilotmega`](https://mavlink.io/en/messages/common.html) — feature-full dialect used by
  [ArduPilot](http://ardupilot.org). In most cases this dialect is the go-to choice if you want to recognize almost
  all MAVLink messages used by existing flight stacks.
* [`all`](https://mavlink.io/en/messages/all.html) — meta-dialect which includes all other standard dialects
  including these which were created for testing purposes. It is guaranteed that namespaces of the dialects in `all`
  family do not collide.
* Other dialects from MAVLink
  XML [definitions](https://github.com/mavlink/mavlink/tree/master/message_definitions/v1.0):
  `asluav`, `avssuas`, `csairlink`, `cubepilot`, `development`, `icarous`, `matrixpilot`, `paparazzi`, `ualberta`,
  `uavionix`. These do not include `python_array_test` and `test` dialects which should be either generated manually
  or as a part of `all` meta-dialect.

For example:

```rust
use mavio::dialects::common as dialect;
use dialect::{Common, messages::Heartbeat};
use mavio::prelude::*;

fn main() -> mavio::error::Result<()> {
    let frame: Frame<V2> = Frame::builder()
        .version(V2).system_id(1).component_id(0).sequence(0)
        .message(&Heartbeat::default())?
        .build();

    // Decode MavLink frame into a dialect message:
    match frame.decode()? {
        Common::Heartbeat(msg) => {
            /* process heartbeat */
        }
        _ => { unreachable!(); }
    };

    Ok(())
}
```

For small applications that use only a small subset of messages, avoid using dialect enums as they contain all message
variants. Instead, decode messages directly from frames:

```rust
use mavio::dialects::common as dialect;
use dialect::messages::Heartbeat;
use mavio::prelude::*;

fn main() -> mavio::error::Result<()> {
    let frame: Frame<V2> = Frame::builder()
        .version(V2).system_id(1).component_id(0).sequence(0)
        .message(&Heartbeat::default())?
        .build();

    // Use only specific messages:
    match frame.message_id() {
        Heartbeat::ID => {
            let msg = Heartbeat::try_from(frame.payload())?;
            /* process heartbeat */
        }
        /* process other messages */
        _ => { unreachable!(); }
    };

    Ok(())
}
```

### Custom Dialects

Actual implementations of MAVLink dialects are generated by [MAVSpec](https://gitlab.com/mavka/libs/mavspec).
There are three ways to generate your own dialects: generate from XML definitions using build script, patch
[`mavlink-message-definitions`](https://crates.io/crates/mavlink-message-definitions), and create your own ad-hoc
dialects. All these options are technically not related to Mavio and are part of MAVSpec. We suggest to check its
[documentation](https://docs.rs/mavspec/latest/mavspec/) for details.

You may also found useful to review [`build.rs`](examples/custom/build.rs) in one of the examples.

### Microservices

Mavio allows to generate additional structures tailored for MAVLink [microservices](https://mavlink.io/en/services/).
Each microservice is a subdialect with only those messages and enums which are necessary. To generate microservice
subdialects use `msrv-*` feature flags.

Mavio also provides additional utils to work with MAVLink microservices. These tools can be enabled by `msrv-utils-*`
feature flags and available in `microservices` module.

Microservice utils feature flags:

- `msrv-utils-all` — all microservice utils.
- `msrv-utils-mission` — MAVLink
  [mission protocol](https://mavlink.io/en/services/mission.html) utils including support for unofficial
  [mission file format](https://mavlink.io/en/file_formats/#mission_plain_text_file).

### Message definitions

It is possible to bundle message definitions generated by [MAVInspect](https://crates.io/crates/mavinspect)
into `definitions` module. This can be useful for ground control stations that require to present the
user with the descriptions of MAVLink entities.

To enable definitions bundling use `definitions` feature flag.

> ⚠️ Message definitions available only with `std` feature enabled. Otherwise, this will cause build to fail.

Examples
--------

Examples for synchronous I/O in [`./examples/sync/examples`](./examples/sync/examples):

* [`tcp_server.rs`](./examples/sync/examples/tcp_server.rs) is a simple TCP server that awaits for connections, sends
  and receives heartbeats:
  ```shell
  cargo run --package mavio_examples_sync --example tcp_server
  ```
* [`tcp_client.rs`](./examples/sync/examples/tcp_client.rs) is a TCP client which connects to server, sends and receives
  heartbeats:
  ```shell
  cargo run --package mavio_examples_sync --example tcp_client
  ```
* [`tcp_ping_pong.rs`](./examples/sync/examples/tcp_ping_pong.rs) server and clients which communicate with each other
  via TCP:
  ```shell
  cargo run --package mavio_examples_sync --example tcp_ping_pong
  ```

Examples for asynchronous I/O in [`./examples/async/examples`](./examples/async/examples):

* [`async_tcp_ping_pong.rs`](./examples/async/examples/async_tcp_ping_pong.rs) server and clients which communicate with
  each other via TCP:
  ```shell
  cargo run --package mavio_examples_async --example async_tcp_ping_pong
  ```

Examples for custom dialect generation with filtered MAVLink entities can be found in
[`./examples/custom/examples`](examples/custom/examples):

* [`custom_dialects_usage.rs`](examples/custom/examples/custom_dialects_usage.rs) a basic usage of
  custom-generated dialect:
  ```shell
  cargo run --package mavio_examples_custom --example mavio_examples_custom_usage
  ```
* [`custom_message.rs`](examples/custom/examples/custom_message.rs) creating and using a custom message:
  ```shell
  cargo run --package mavio_examples_custom --example custom_message
  ```

License
-------

> Here we simply comply with the suggested dual licensing according to
> [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html) (C-PERMISSIVE).

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
