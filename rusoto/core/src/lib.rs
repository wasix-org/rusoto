#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(
    feature = "nightly-testing",
    allow(
        cyclomatic_complexity,
        used_underscore_binding,
        ptr_arg,
        suspicious_else_formatting
    )
)]
#![allow(dead_code)]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]
#![deny(missing_docs)]

//! Rusoto is an [AWS](https://aws.amazon.com/) SDK for Rust.
//! A high level overview is available in `README.md` at <https://github.com/rusoto/rusoto>.

mod client;
mod error;
mod stream;

pub mod event_stream;
pub mod param;
#[doc(hidden)]
pub mod region;
pub mod request;
#[doc(hidden)]
pub mod signature;

pub use crate::client::Client;
#[doc(hidden)]
pub mod encoding;
#[doc(hidden)]
pub mod proto;
#[doc(hidden)]
pub mod serialization;

pub use crate::error::{RusotoError, RusotoResult};
pub use crate::region::Region;
pub use crate::request::{DispatchSignedRequest, HttpClient, HttpConfig, HttpDispatchError};
pub use crate::stream::ByteStream;
pub use rusoto_credential as credential;
