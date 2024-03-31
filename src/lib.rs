//! Library for [archive files][] like `.tar`.
//!
//! [archive files]: https://en.wikipedia.org/wiki/Archive_file

#![forbid(unsafe_code)]
#![deny(clippy::all, missing_docs, unused_must_use)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

#[cfg(not(any(feature = "tar")))]
compile_error!("there must be at least one archive file format feature");

#[cfg(all(feature = "bzip2", feature = "bzip2-rs"))]
compile_error!("there must be only one bzip2 feature");

mod archive;
mod entries;
mod entry;
mod error;
pub mod support;

pub use archive::Archive;
pub use entries::Entries;
pub use entry::Entry;
pub use entry::EntryType;
pub use error::Error;
pub(crate) use error::Result;
