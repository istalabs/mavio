//! # Mavio
//!
//! Minimalistic library for transport-agnostic [MAVLink](https://mavlink.io/en/) communication.
//! It supports `no-std` (and `no-alloc`) targets.
//!
//! <span style="font-size:24px">[🇺🇦](https://mavka.gitlab.io/home/a_note_on_the_war_in_ukraine/)</span>
//! [![`repository`](https://img.shields.io/gitlab/pipeline-status/mavka/libs/mavio.svg?logo=gitlab&branch=main&label=repository)](https://gitlab.com/mavka/libs/mavio)
//! [![`crates.io`](https://img.shields.io/crates/v/mavio.svg)](https://crates.io/crates/mavio)
//! [![`docs.rs`](https://img.shields.io/docsrs/mavio.svg?label=docs)](https://docs.rs/mavio/latest/mavio/)
//! [![`issues`](https://img.shields.io/gitlab/issues/open/mavka/libs/mavio.svg)](https://gitlab.com/mavka/libs/mavio/-/issues/)
//!
//! This library is a part of [Mavka](https://mavka.gitlab.io/home/) toolchain. It uses
//! [MAVSpec](https://gitlab.com/mavka/libs/mavspec) to generate MAVLink dialects.
//!
//! ## Usage
//!
//! This library exposes [`Sender`] and [`Receiver`] to send and receive instances of MAVLink
//! [`Frame`]. Frames contain encoded message body in [`Frame::payload`] and additional fields (such
//! as `sequence` or `system_id`) as required by [MAVLink specification](https://mavlink.io/en/guide/serialization.html).
//! Once frame is received, it can be decoded into a specific `Message`. Frame decoding requires
//! dialect specification which can be either generated manually by using
//! [MAVSpec](https://gitlab.com/mavka/libs/mavspec) or by enabling built-in
//! [dialect features](#dialects).
//!
//! Upon receiving or building a frame, it can be converted into a protocol-agnostic [`MavFrame`],
//! that hides generic version parameter of a [`Frame`].
//!
//! ### Receive
//!
//! Connect to TCP port and receive first 10 MAVLink frames, decode any received
//! [HEARTBEAT](https://mavlink.io/en/messages/common.html#HEARTBEAT) messages.
//!
//! ```rust,no_run
//! # #[cfg(not(all(feature = "minimal", feature = "std")))]
//! # fn main() {}
//! # #[cfg(all(feature = "minimal", feature = "std"))]
//! # fn main() -> mavio::error::Result<()> {
//! use std::net::TcpStream;
//!
//! use mavio::prelude::*;
//! use mavio::dialects::minimal as dialect;
//!
//! use dialect::Minimal;
//!
//! // Create a TCP client receiver
//! let reader = StdIoReader::new(TcpStream::connect("0.0.0.0:5600")?);
//! let mut receiver = Receiver::versionless(reader);
//!
//! for i in 0..10 {
//!     let frame = receiver.recv()?;
//!
//!     // Validate MAVLink frame
//!     if let Err(err) = frame.validate_checksum::<Minimal>() {
//!         eprintln!("Invalid checksum: {err:?}");
//!         continue;
//!     }
//!
//!     if let Ok(Minimal::Heartbeat(msg)) = frame.decode() {
//!         println!(
//!             "HEARTBEAT #{}: mavlink_version={:#?}",
//!             frame.sequence(),
//!             msg.mavlink_version,
//!         );
//!     }
//! }
//!
//! # Ok(())
//! # }
//! ```
//!
//! ### Send
//!
//! Connect to TCP port and send 10 [HEARTBEAT](https://mavlink.io/en/messages/common.html#HEARTBEAT) messages using
//! `MAVLink 2` protocol.
//!
//! ```rust,no_run
//! # #[cfg(not(all(feature = "minimal", feature = "std")))]
//! # fn main() {}
//! # #[cfg(all(feature = "minimal", feature = "std"))]
//! # fn main() -> mavio::error::Result<()> {
//! use std::net::TcpStream;
//!
//! use mavio::prelude::*;
//! use mavio::dialects::minimal as dialect;
//! use dialect::enums::{MavAutopilot, MavModeFlag, MavState, MavType};
//!
//! // Create a TCP client sender
//! let writer = StdIoWriter::new(TcpStream::connect("0.0.0.0:5600")?);
//! let mut sender = Sender::new(writer);
//!
//! // Create an endpoint that represents a MAVLink device speaking `MAVLink 2` protocol
//! let endpoint = Endpoint::v2(MavLinkId::new(15, 42));
//!
//! // Create a message
//! let message = dialect::messages::Heartbeat {
//!     type_: MavType::FixedWing,
//!     autopilot: MavAutopilot::Generic,
//!     base_mode: MavModeFlag::TEST_ENABLED & MavModeFlag::CUSTOM_MODE_ENABLED,
//!     custom_mode: 0,
//!     system_status: MavState::Active,
//!     mavlink_version: dialect::Minimal::version().unwrap(),
//! };
//! println!("MESSAGE: {message:?}");
//!
//! for i in 0..10 {
//!     // Build the next frame for this endpoint.
//!     // All required fields will be populated, including frame sequence counter.
//!     let frame = endpoint.next_frame(&message)?;
//!
//!     sender.send(&frame)?;
//!     println!("FRAME #{} sent: {:#?}", i, frame);
//! }
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! This library is a building block for more sophisticated tools. It includes absolute minimum of functionality
//! required for correct communication with everything that speaks MAVLink protocol:
//!
//! * It supports both `MAVLink 1` and `MAVLink 2` protocol versions.
//! * Provides intermediate MAVLink packets decoding with [`Frame`] that contain only header, checksum and signature
//!   being deserialized. Which means that client don't have to decode the entire message for routing and verification.
//! * Supports optional high-level message decoding by utilizing MAVLink abstractions generated by
//!   [MAVSpec](https://gitlab.com/mavka/libs/mavspec).
//! * Includes standard MAVLink dialects enabled by cargo features.
//! * Implements message verification via checksum.
//! * Provides a mechanism for message sequencing through [`Endpoint::next_frame`], that encodes a
//!   MAVLink message into a [`Frame`] with a correct sequence as required by MAVLink
//!   [protocol](https://mavlink.io/en/guide/serialization.html).
//! * Includes tools for [message signing](https://mavlink.io/en/guide/message_signing.html).
//!
//! ### Extra features
//!
//! Most of the "extra" features are related to decoupling from MAVLink XML definitions parsing and code generation.
//! These tasks are performed by [MAVInspect](https://gitlab.com/mavka/libs/mavinspect) and
//! [MAVSpec](https://gitlab.com/mavka/libs/mavspec) respectively.
//!
//! * Supports custom dialects or may work with no dialect at all (for intermediate decoding).
//! * Includes support for custom payload decoders and encoders. Which means that clients are not
//!   bounded by abstractions generated by [MAVSpec](https://gitlab.com/mavka/libs/mavspec).
//! * Uses implementation-agnostic I/O primitives with a variety of I/O [adapters](io::adapters).
//! * Compatible with `no_std` targets.
//! * Provides synchronous I/O adapters for [embedded-io](https://docs.rs/embedded-io/) and
//!   [`std::io`].
//! * Supports asynchronous I/O by providing lightweight adapters for
//!   [embedded-io-async](https://docs.rs/embedded-io-async/), [Tokio](https://tokio.rs), and
//!   [futures-rs](https://docs.rs/futures/).
//! * Respects dialect inheritance. Messages defined in one dialect are not redefined upon inclusion
//!   into another dialect. This means that if you have a message `M` from dialect `A` being
//!   included by dialect `B`, it guaranteed that you can use Rust structs for message `M` with both
//!   of the dialects. The same is true for MAVLink enums and bitmasks.
//!
//! ### Out of scope
//!
//! There are few *stateful* features required by MAVLink protocol this library intentionally does
//! not implement and leaves for the client:
//!
//! * Sending automatic [heartbeats](https://mavlink.io/en/services/heartbeat.html). This is
//!   required by most of the clients which would consider nodes without heartbeat updates as
//!   inactive or invalid.
//! * Stateful timestamp management for [message signing](https://mavlink.io/en/guide/message_signing.html)
//!   ensuring, that two messages are not sent with the same timestamp.
//!
//! ## I/O
//!
//! We provide lightweight [`Read`](io::Read) / [`Write`](io::Write) and
//! [`AsyncRead`](io::AsyncRead) / [`AsyncWrite`](io::AsyncWrite) pairs for synchronous and
//! asynchronous I/O.
//!
//! Mavio packages a set of I/O adapters available under the corresponding feature flags:
//!
//! - `embedded-io` → [`io::EmbeddedIoReader`] / [`io::EmbeddedIoWriter`]
//! - `embedded-io-async` → [`io::EmbeddedIoAsyncReader`] / [`io::EmbeddedIoAsyncWriter`]
//! - `std` → [`io::StdIoReader`] / [`io::StdIoWriter`]
//! - `tokio` → [`io::TokioReader`] / [`io::TokioWriter`]
//! - `futures` → [`io::FuturesReader`] / [`io::FuturesWriter`]
//!
//! See [`adapters`](io::adapters) for details.
//!
//! ## Dialects
//!
//! Standard MAVLink dialect can be enabled by the corresponding feature flags.
//!
//! * [`minimal`]((https://mavlink.io/en/messages/minimal.html)) — minimal dialect required to
//!   expose your presence to other MAVLink devices.
//! * [`standard`](https://mavlink.io/en/messages/standard.html) — a superset of `minimal` dialect,
//!   that expected to be used by almost all flight stack.
//! * [`common`](https://mavlink.io/en/messages/common.html) — minimum viable dialect with most of
//!   the features, a building block for other future-rich dialects.
//! * [`ardupilotmega`](https://mavlink.io/en/messages/common.html) — feature-full dialect used by
//!   [ArduPilot](http://ardupilot.org). In most cases this dialect is the go-to choice if you want
//!   to recognize almost all MAVLink messages used by existing flight stacks.
//! * [`all`](https://mavlink.io/en/messages/all.html) — meta-dialect which includes all other
//!   standard dialects including these which were created for testing purposes. It is guaranteed
//!   that namespaces of the dialects in `all` family do not collide.
//! * Other dialects from MAVLink XML [definitions](https://github.com/mavlink/mavlink/tree/master/message_definitions/v1.0):
//!   `asluav`, `avssuas`, `csairlink`, `cubepilot`, `development`, `icarous`, `matrixpilot`,
//!   `paparazzi`, `ualberta`, `uavionix`. These do not include `python_array_test` and `test`
//!   dialects which should be either generated manually or as a part of `all` meta-dialect.
//!
//! For example:
//!
//! ```rust
//! # #[cfg(not(all(feature = "common", feature = "std")))]
//! # fn main() {}
//! # #[cfg(all(feature = "common", feature = "std"))]
//! # fn main() -> mavio::error::Result<()> {
//! use mavio::dialects::common as dialect;
//! use dialect::{Common, messages::Heartbeat};
//! use mavio::prelude::*;
//!
//! let frame: Frame<V2> = /* obtain a frame */
//! #    Frame::builder()
//! #        .version(V2)
//! #        .system_id(1)
//! #        .component_id(0)
//! #        .sequence(0)
//! #        .message(&Heartbeat::default())?
//! #        .build();
//!
//! // Decode MavLink frame into dialect messages:
//! match frame.decode()? {
//!     Common::Heartbeat(msg) => {
//!         /* process heartbeat */
//!     }
//!     /* process other messages */
//!     # _ => { unreachable!(); }
//! };
//! # Ok(())
//! # }
//! ```
//!
//! ## Unsafe Features
//!
//! This library does not use unsafe Rust, however, certain manipulations on MAVLink frames, if not
//! performed carefully, could lead to data corruption and undefined behavior. All such operations
//! are covered by `unsafe` cargo features and marked with <sup>`⚠`</sup> in the documentation.
//!
//! Most of the unsafe operations are related to updating existing frames in-place. In general,
//! situations when you need mutable access to frames are rare. If your use-case does not strictly
//! require such manipulations, we suggest to refrain from using functionality hidden under the
//! `unsafe` feature flags.
//!
//! ## Incompatible Features
//!
//! - [Specta](https://crates.io/crates/specta) requires `std` feature to be enabled.
//!
//! ## Binary Size
//!
//! For small applications that use only a small subset of messages, avoid using dialect enums as
//! they contain all message variants. Instead, decode messages directly from frames:
//!
//! ```rust
//! # #[cfg(not(all(feature = "common", feature = "std")))]
//! # fn main() {}
//! # #[cfg(all(feature = "common", feature = "std"))]
//! # fn main() -> Result<(), mavio::error::SpecError> {
//! use mavio::dialects::common as dialect;
//! use dialect::messages::Heartbeat;
//! use mavio::prelude::*;
//!
//! let frame: Frame<V2> = /* obtain a frame */
//! #    Frame::builder()
//! #        .version(V2)
//! #        .system_id(1)
//! #        .component_id(0)
//! #        .sequence(0)
//! #        .message(&Heartbeat::default()).unwrap()
//! #        .build();
//!
//! // Use only specific messages:
//! match frame.message_id() {
//!     Heartbeat::ID => {
//!         let msg = Heartbeat::try_from(frame.payload())?;
//!         /* process heartbeat */
//!     }
//!     /* process other messages */
//!     # _ => { unreachable!(); }
//! };
//! # Ok(())
//! # }
//! ```
//!
//! This will help compiler to throw away unnecessary pieces of code.

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(
    html_logo_url = "https://gitlab.com/mavka/libs/mavio/-/raw/main/avatar.png?ref_type=heads",
    html_favicon_url = "https://gitlab.com/mavka/libs/mavio/-/raw/main/avatar.png?ref_type=heads"
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate core;

pub mod consts;
pub mod error;
pub mod io;
pub mod prelude;
pub mod protocol;
pub mod utils;

#[cfg(feature = "tokio")]
#[doc(inline)]
pub use crate::io::{AsyncReceiver, AsyncSender};
#[doc(inline)]
pub use crate::io::{Receiver, Sender};
#[doc(inline)]
pub use error::Result;
#[doc(inline)]
pub use protocol::{Dialect, Endpoint, Frame, MavFrame, MavLinkId, Message};

mod mavlink {
    include!(concat!(env!("OUT_DIR"), "/mavlink/mod.rs"));
}
pub use mavlink::dialects;

#[cfg(all(feature = "specta", not(feature = "std")))]
compile_error!("Specta support is currently available only for `std` targets! Add `std` feature.");
