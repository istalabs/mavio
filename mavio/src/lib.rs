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
//! This library is a part of [Mavka](https://mavka.gitlab.io/home/) toolchain. It focuses on I/O
//! and uses [MAVSpec](https://gitlab.com/mavka/libs/mavspec) to generate MAVLink dialects.
//!
//! # Features
//!
//! Mavio is focused on I/O primitives for MAVLink that can be used in `no-std` and `no-alloc`
//! environments. Since Mavio is designed as a building block for more sophisticated tools,
//! it includes absolute minimum of functionality required for correct communication with everything
//! that speaks MAVLink protocol.
//!
//! ## Basic capabilities
//!
//! * Mavio supports both `MAVLink 1` and `MAVLink 2` protocol versions.
//! * Provides intermediate MAVLink packets decoding with [`Frame`] that contain only header, checksum and signature
//!   being deserialized. Which means that client don't have to decode the entire message for routing and verification.
//! * Supports optional high-level message decoding by utilizing MAVLink abstractions generated by
//!   [MAVSpec](https://crates.io/crates/mavspec).
//! * Includes standard MAVLink dialects enabled by cargo features.
//! * Implements message verification via checksum.
//! * Provides a mechanism for message sequencing through [`Endpoint::next_frame`], that encodes a
//!   MAVLink message into a [`Frame`] with a correct sequence as required by MAVLink
//!   [protocol](https://mavlink.io/en/guide/serialization.html).
//! * Includes tools for [message signing](https://mavlink.io/en/guide/message_signing.html).
//!
//! ## Flexible approach to I/O
//!
//! Mavio is designed to be flexible and useful in different scenarios.
//!
//! * It can work without any MAVLink dialect at all (for intermediate decoding and proxying).
//! * Includes support for custom payload decoders and encoders. Which means that clients are not
//!   bounded by abstractions generated by [MAVSpec](https://crates.io/crates/mavspec).
//! * Uses implementation-agnostic I/O primitives with a variety of I/O [adapters](io::adapters).
//! * Provides synchronous I/O adapters for [embedded-io](https://docs.rs/embedded-io/) and
//!   [`std::io`].
//! * Supports asynchronous I/O by providing lightweight adapters for
//!   [embedded-io-async](https://docs.rs/embedded-io-async/), [Tokio](https://tokio.rs), and
//!   [futures-rs](https://docs.rs/futures/).
//!
//! ## MAVLink protocol support
//!
//! Such features are mainly related to the functionality provided by other parts of
//! [Mavka](https://mavka.gitlab.io/home/) toolchain. These tasks are mainly performed by
//! [MAVInspect](https://crates.io/crates/mavinspect) and [MAVSpec](https://crates.io/crates/mavspec)
//! (a message definition parser and code-generator respectively). In particular:
//!
//! * Mavio allows to use custom MAVLink dialects. This includes both dialects generated from XML
//!   definitions and ad-hoc dialects defined with pure Rust.
//! * Respects dialect inheritance. Messages defined in one dialect are not redefined upon inclusion
//!   into another dialect. This means that if you have a message `M` from dialect `A` being
//!   included by dialect `B`, it guaranteed that you can use Rust structs for message `M` with both
//!   of the dialects. The same is true for MAVLink enums and bitmasks.
//! * Provides optional support for MAVLink [microservices](https://mavlink.io/en/services/) as
//!   sub-dialects and helpers to work with them such as
//!   [mission file format](https://mavlink.io/en/file_formats/#mission_plain_text_file).
//!
//! ## Serialization / deserialization and FFI
//!
//! Mavio provides support for [serde](https://serde.rs) and [specta](https://specta.dev). The
//! latter is supported only for `std` targets. All MAVLink entities are fully supported.
//!
//! ## Out of scope
//!
//! There are few *stateful* features required by MAVLink protocol this library intentionally does
//! not implement and leaves for the client:
//!
//! * Sending automatic [heartbeats](https://mavlink.io/en/services/heartbeat.html). This is
//!   required by most of the clients which would consider nodes without heartbeats as inactive or
//!   invalid.
//! * Stateful timestamp management for [message signing](https://mavlink.io/en/guide/message_signing.html)
//!   (ensuring, that two messages are not sent with the same timestamp).
//! * Retry logic for failed connections.
//!
//! All such features are provided by [Maviola](https://crates.io/crates/maviola).
//!
//! # Usage
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
//! ## Receive
//!
//! Connect to TCP port and receive first 10 MAVLink frames, decode any received
//! [HEARTBEAT](https://mavlink.io/en/messages/common.html#HEARTBEAT) messages.
//!
//! ```rust,no_run
//! # #[cfg(not(all(feature = "dlct-minimal", feature = "std")))]
//! # fn main() {}
//! # #[cfg(all(feature = "dlct-minimal", feature = "std"))]
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
//! ## Send
//!
//! Connect to TCP port and send 10 [HEARTBEAT](https://mavlink.io/en/messages/common.html#HEARTBEAT) messages using
//! `MAVLink 2` protocol.
//!
//! ```rust,no_run
//! # #[cfg(not(all(feature = "dlct-minimal", feature = "std")))]
//! # fn main() {}
//! # #[cfg(all(feature = "dlct-minimal", feature = "std"))]
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
//! # I/O adapters
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
//! # MAVLink protocol
//!
//! We use [MAVSpec](https://crates.io/crates/mavspec) to generate MAVLink entities and additional
//! abstractions. These entities are bundled with Mavio for better user experience and to prevent
//! discrepancies in crate versions.
//!
//! ## Dialects
//!
//! Standard MAVLink dialect can be enabled by the corresponding feature flags (`dlct-<dialect name>`).
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
//!   standard dialects including those which were created for testing purposes. It is guaranteed
//!   that namespaces of the dialects in `all` family do not collide.
//! * Other dialects from MAVLink XML [definitions](https://github.com/mavlink/mavlink/tree/master/message_definitions/v1.0):
//!   `asluav`, `avssuas`, `csairlink`, `cubepilot`, `development`, `icarous`, `matrixpilot`,
//!   `paparazzi`, `ualberta`, `uavionix`. These do not include `python_array_test` and `test`
//!   dialects which should be either generated manually or as a part of `all` meta-dialect.
//!
//! For example:
//!
//! ```rust
//! # #[cfg(not(all(feature = "dlct-common", feature = "std")))]
//! # fn main() {}
//! # #[cfg(all(feature = "dlct-common", feature = "std"))]
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
//! // Decode MavLink frame into a dialect message:
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
//! ## Default dialect
//!
//! When standard MAVLink dialects are used and at least `dlct-minimal` Cargo feature is enabled,
//! this library exposes [`default_dialect`] and [`DefaultDialect`] entities that allow to access
//! the most feature-rich enabled MAVLink dialect. Other features such as [`microservices`] or
//! [microservice utils](microservices::utils) are based on this convention.
//!
//! The sequence of default dialects is the following (in the order of the increased completeness):
//!
//! - [`minimal`](dialects::minimal) — enabled by `dlct-minimal` feature flag
//! - [`standard`](dialects::standard) — enabled by `dlct-standard` feature flag
//! - [`common`](dialects::common) — enabled by `dlct-common` feature flag
//! - [`ardupilotmega`](dialects::ardupilotmega) — enabled by `dlct-ardupilotmega` feature flag
//! - [`all`](dialects::all) — enabled by `dlct-all` feature flag
//!
//! ## Microservices
//!
//! Mavio re-exports MAVLink [microservices](https://mavlink.io/en/services/) from
//! [MAVSpec](https://crates.io/crates/mavspec). To control which microservices you want to generate,
//! use `msrv-*` feature flags family. Check MAVSpec [API docs](https://docs.rs/mavspec/latest) for
//! details.
//!
//! At the moment, microservices are generated only for [`default_dialect`] and can be accessed
//! through [`microservices`].
//!
//! ### Microservice utils
//!
//! In addition, Mavio re-exports extra tools for working with microservices. These tools can be
//! enabled by `msrv-utils-*` feature flags and available in [`microservices::utils`] module and
//! bundled inside the corresponding microservice.
//!
//! <section class="warning">
//! `msrv-utils-*` are considered unstable for now! Use `unstable` feature flag to enable them.
//! </section>
//!
//! ## Message definitions
//!
//! It is possible to bundle message definitions generated by [MAVInspect](https://crates.io/crates/mavinspect)
//! into [`definitions`] module. This can be useful for ground control stations that require to present the
//! user with the descriptions of MAVLink entities.
//!
//! To enable definitions bundling use `definitions` feature flag.
//!
//! <section class="warning">
//! Message definitions available only with `std` feature enabled. Otherwise, this will cause build
//! to fail.
//! </section>
//!
//! ## Metadata
//!
//! MAVSpec can generate additional metadata such as MAVLink enum entry names. This can be a useful
//! addition to MAVlink [`definitions`] when ground stations are considered. To enable metadata
//! support, use `metadata` feature flag.
//!
//! # Caveats
//!
//! The API is straightforward and generally stable, however, some caution is required when working
//! with edge-cases.
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
//! ## Unstable Features
//!
//! Certain features are considered unstable and available only when `unstable` feature flag is
//! enabled. Unstable features are marked with <sup>`⍚`</sup> and are may be changed in futures
//! versions.
//!
//! ## Incompatible Features
//!
//! - [Specta](https://crates.io/crates/specta) requires `std` feature to be enabled.
//! - MAVlink [`definitions`] requires `std` feature to be enabled.
//!
//! ## Binary Size
//!
//! For small applications that use only a small subset of messages, avoid using dialect enums as
//! they contain all message variants. Instead, decode messages directly from frames:
//!
//! ```rust
//! # #[cfg(not(all(feature = "dlct-common", feature = "std")))]
//! # fn main() {}
//! # #[cfg(all(feature = "dlct-common", feature = "std"))]
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
//!
//! Additionally, you may use [`microservices`] as well as [`microservices::utils`] to reduce API
//! surface you are interacting with.
//!
//! # Feature flags
//!
//! In most of the cases you will be interested in `dlct-*` features to access MAVLink [`dialects`],
//! I/O [`adapters`](io::adapters), and `alloc` / `std` target specification. However, a more
//! fine-grained control may be required.
#![cfg_attr(feature = "std", doc = document_features::document_features!())]
//
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
pub use error::{Error, Result};
#[doc(inline)]
pub use protocol::{Dialect, Endpoint, Frame, MavFrame, MavLinkId, Message};

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
/// Default MAVLink dialect module
///
/// Similar to [`DefaultDialect`] but provides access to a dialect module instead of dialect itself.
///
/// See [`DefaultDialect`] to learn about logic behind choosing a default dialect.
///
/// # Usage
///
/// ```rust,no_run
/// use mavio::default_dialect;
///
/// let message = default_dialect::messages::Heartbeat::default();
/// ```
///
/// Requires at least `dlct-minimal` dialect feature flag to be enabled.
///
/// Re-exported from [`mavspec::rust::default_dialect`].
///
/// ---
#[cfg(feature = "dlct-minimal")]
#[doc(inline)]
pub use mavspec::rust::default_dialect;

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
/// Default MAVLink dialect
///
/// The rules for determining the default dialect are defined by the following order of canonical dialect inclusion:
///
/// [`all`](https://mavlink.io/en/messages/all.html) >
/// [`ardupilotmega`](https://mavlink.io/en/messages/common.html) >
/// [`common`](https://mavlink.io/en/messages/common.html) >
/// [`standard`]((https://mavlink.io/en/messages/standard.html))
/// [`minimal`]((https://mavlink.io/en/messages/minimal.html))
///
/// That means, that if you enabled `dlct-ardupilotmega` dialect but not `all`, then the former is the
/// most general canonical dialect, and it will be chosen as a default one.
///
/// Requires at least `dlct-minimal` dialect feature flag to be enabled.
///
/// Re-exported from [`mavspec::rust::DefaultDialect`].
///
/// ---
#[cfg(feature = "dlct-minimal")]
#[doc(inline)]
pub use mavspec::rust::DefaultDialect;

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
/// MAVLink dialects
///
/// These dialects are generated by [MAVSpec](https://crates.io/crates/mavspec).
///
/// Each dialect belongs to a specific module, such as:
///
/// - [`minimal`](crate::dialects::minimal)
/// - [`common`](crate::dialects::common)
/// - [`ardupilotmega`](crate::dialects::ardupilotmega)
/// - ... and so on
///
/// Re-exported from [`mavspec::rust::dialects`].
///
/// ---
#[doc(inline)]
pub use mavspec::rust::dialects;

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
/// MAVLink [microservices](https://mavlink.io/en/services/)
///
/// Enabled by `msrv-*` feature flags, additional tools are available as [`microservices::utils`]
/// via `msrv-utils-*` feature flags (requires `unstable` feature).
///
/// Re-exported from [`mavspec::rust::microservices`].
///
/// ---
#[cfg(all(feature = "msrv", feature = "dlct-minimal"))]
#[doc(inline)]
pub use mavspec::rust::microservices;

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
/// MAVLink message definitions
///
/// Requires `definitions` feature flag to be enabled.
///
/// <section class="warning">
/// Requires `std` feature flag to be enabled. Otherwise, the library won't compile.
/// </section>
///
/// Re-exported from [`mavspec::definitions`].
///
/// ---
#[cfg(feature = "definitions")]
#[doc(inline)]
pub use mavspec::definitions;

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
/// MAVSpec procedural macros
///
/// Since derive macros relies on entities from [`mavspec::rust::spec`], you have to import
/// [`mavspec`] or use [`prelude`]. For example:
///
/// ```rust
/// #[cfg(feature = "derive")]
/// # {
/// use mavio::prelude::*; // This is necessary!!!
/// use mavio::derive::Enum;
///
/// #[derive(Enum)]
/// #[repr(u8)]
/// #[derive(Copy, Clone, Debug, Default)]
/// enum CustomEnum {
///     #[default]
///     DEFAULT = 0,
///     OptionA = 1,
///     OptionB = 2,
/// }
/// # }
/// ```
///
/// Requires `derive` feature flag to be enabled.
///
/// Re-exported from [`mavspec::rust::derive`].
///
/// ---
#[cfg(feature = "derive")]
#[doc(inline)]
pub use mavspec::rust::derive;

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
/// [MAVSpec](https://crates.io/crates/mavspec) re-exported
///
/// We re-export MAVSpec in order to simplify interoperability with the tools provided by this
/// library.
///
/// For example, [`derive`](mod@derive) proc macros depends on [`mavspec::rust::spec`] being
/// accessible.
///
/// ---
#[doc(inline)]
pub use mavspec;

#[cfg(all(feature = "specta", not(feature = "std")))]
compile_error!("Specta support is currently available only for `std` targets! Add `std` feature.");

#[cfg(all(feature = "definitions", not(feature = "std")))]
compile_error!(
    "MAVLink definitions support is currently available only for `std` targets! Add `std` feature."
);
