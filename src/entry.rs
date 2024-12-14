use std::borrow::Cow;
#[cfg(feature = "tar")]
use std::fs::File;
#[cfg(feature = "zstd")]
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

#[cfg(feature = "bzip2")]
use bzip2::read::BzDecoder as BzSysDecoder;
#[cfg(feature = "bzip2-rs")]
use bzip2_rs::decoder::DecoderReader as BzNativeDecoder;
#[cfg(feature = "flate2")]
use flate2::read::GzDecoder;
#[cfg(feature = "lz4")]
use lz4::Decoder as Lz4Decoder;
#[cfg(feature = "xz2")]
use xz2::read::XzDecoder;
#[cfg(feature = "zstd")]
use zstd::stream::read::Decoder as ZstdDecoder;

use crate::Result;

/// Archive entry.
// NONEXHAUSTIVE new formats could add new types
#[non_exhaustive]
pub enum Entry<'a> {
    #[cfg(feature = "tar")]
    #[doc(hidden)]
    Tar(tar::Entry<'a, File>),

    #[cfg(all(feature = "bzip2", feature = "tar"))]
    #[doc(hidden)]
    TarBzip2(tar::Entry<'a, BzSysDecoder<File>>),

    #[cfg(all(feature = "bzip2-rs", feature = "tar"))]
    #[doc(hidden)]
    TarBzip2Rs(tar::Entry<'a, BzNativeDecoder<File>>),

    #[cfg(all(feature = "flate2", feature = "tar"))]
    #[doc(hidden)]
    TarGzip(tar::Entry<'a, GzDecoder<File>>),

    #[cfg(all(feature = "lz4", feature = "tar"))]
    #[doc(hidden)]
    TarLz4(tar::Entry<'a, Lz4Decoder<File>>),

    #[cfg(all(feature = "xz2", feature = "tar"))]
    #[doc(hidden)]
    TarXz(tar::Entry<'a, XzDecoder<File>>),

    #[cfg(all(feature = "zstd", feature = "tar"))]
    #[doc(hidden)]
    TarZstd(tar::Entry<'a, ZstdDecoder<'static, BufReader<File>>>),

    #[cfg(not(feature = "tar"))]
    #[doc(hidden)]
    __Phantom(std::marker::PhantomData<&'a str>),
}

impl Entry<'_> {
    /// Returns the file type of this entry.
    pub fn entry_type(&self) -> EntryType {
        match self {
            #[cfg(feature = "tar")]
            Self::Tar(entry) => entry.header().entry_type().into(),

            #[cfg(all(feature = "bzip2", feature = "tar"))]
            Self::TarBzip2(entry) => entry.header().entry_type().into(),

            #[cfg(all(feature = "bzip2-rs", feature = "tar"))]
            Self::TarBzip2Rs(entry) => entry.header().entry_type().into(),

            #[cfg(all(feature = "flate2", feature = "tar"))]
            Self::TarGzip(entry) => entry.header().entry_type().into(),

            #[cfg(all(feature = "lz4", feature = "tar"))]
            Self::TarLz4(entry) => entry.header().entry_type().into(),

            #[cfg(all(feature = "xz2", feature = "tar"))]
            Self::TarXz(entry) => entry.header().entry_type().into(),

            #[cfg(all(feature = "zstd", feature = "tar"))]
            Self::TarZstd(entry) => entry.header().entry_type().into(),
        }
    }

    /// Returns the size in bytes of the entry.
    #[must_use]
    pub fn size(&self) -> u64 {
        match self {
            #[cfg(feature = "tar")]
            Self::Tar(entry) => entry.size(),

            #[cfg(all(feature = "bzip2", feature = "tar"))]
            Self::TarBzip2(entry) => entry.size(),

            #[cfg(all(feature = "bzip2-rs", feature = "tar"))]
            Self::TarBzip2Rs(entry) => entry.size(),

            #[cfg(all(feature = "flate2", feature = "tar"))]
            Self::TarGzip(entry) => entry.size(),

            #[cfg(all(feature = "lz4", feature = "tar"))]
            Self::TarLz4(entry) => entry.size(),

            #[cfg(all(feature = "xz2", feature = "tar"))]
            Self::TarXz(entry) => entry.size(),

            #[cfg(all(feature = "zstd", feature = "tar"))]
            Self::TarZstd(entry) => entry.size(),
        }
    }

    /// Returns the path name for this entry.
    ///
    /// # Errors
    ///
    /// Parsing the path metadata.
    pub fn path(&self) -> Result<Cow<Path>> {
        match self {
            #[cfg(feature = "tar")]
            Self::Tar(entry) => entry.path().map_err(From::from),

            #[cfg(all(feature = "bzip2", feature = "tar"))]
            Self::TarBzip2(entry) => entry.path().map_err(From::from),

            #[cfg(all(feature = "bzip2-rs", feature = "tar"))]
            Self::TarBzip2Rs(entry) => entry.path().map_err(From::from),

            #[cfg(all(feature = "flate2", feature = "tar"))]
            Self::TarGzip(entry) => entry.path().map_err(From::from),

            #[cfg(all(feature = "lz4", feature = "tar"))]
            Self::TarLz4(entry) => entry.path().map_err(From::from),

            #[cfg(all(feature = "xz2", feature = "tar"))]
            Self::TarXz(entry) => entry.path().map_err(From::from),

            #[cfg(all(feature = "zstd", feature = "tar"))]
            Self::TarZstd(entry) => entry.path().map_err(From::from),
        }
    }
}

impl Read for Entry<'_> {
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> std::result::Result<usize, std::io::Error> {
        match self {
            #[cfg(feature = "tar")]
            Self::Tar(entry) => entry.read(buf),

            #[cfg(all(feature = "bzip2", feature = "tar"))]
            Self::TarBzip2(entry) => entry.read(buf),

            #[cfg(all(feature = "bzip2-rs", feature = "tar"))]
            Self::TarBzip2Rs(entry) => entry.read(buf),

            #[cfg(all(feature = "flate2", feature = "tar"))]
            Self::TarGzip(entry) => entry.read(buf),

            #[cfg(all(feature = "lz4", feature = "tar"))]
            Self::TarLz4(entry) => entry.read(buf),

            #[cfg(all(feature = "xz2", feature = "tar"))]
            Self::TarXz(entry) => entry.read(buf),

            #[cfg(all(feature = "zstd", feature = "tar"))]
            Self::TarZstd(entry) => entry.read(buf),
        }
    }
}

/// File type of an entry.
// ALLOW this is not a public module
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
// NONEXHAUSTIVE new formats could add new entry types
#[non_exhaustive]
#[must_use = "getting an entry type is used to inspect it"]
pub enum EntryType {
    /// Regular file
    Regular,

    /// Hard link
    Link,

    /// Symbolic link
    Symlink,

    /// Character device
    Char,

    /// Block device
    Block,

    /// Directory
    Directory,

    /// Named pipe (fifo)
    Fifo,

    /// Implementation-defined 'high-performance' type, treated as regular file
    Continuous,

    /// GNU extension - long file name
    GNULongName,

    /// GNU extension - long link name (link target)
    GNULongLink,

    /// GNU extension - sparse file
    GNUSparse,

    /// Global extended header
    XGlobalHeader,

    /// Extended Header
    XHeader,

    /// Other
    Other,
}

impl EntryType {
    /// Returns true if this is a regular file.
    #[must_use = "side effect free function"]
    pub fn is_file(&self) -> bool {
        self == &Self::Regular
    }
}

#[cfg(feature = "tar")]
impl From<tar::EntryType> for EntryType {
    fn from(e: tar::EntryType) -> Self {
        match e {
            tar::EntryType::Regular => Self::Regular,
            tar::EntryType::Link => Self::Link,
            tar::EntryType::Symlink => Self::Symlink,
            tar::EntryType::Char => Self::Char,
            tar::EntryType::Block => Self::Block,
            tar::EntryType::Directory => Self::Directory,
            tar::EntryType::Fifo => Self::Fifo,
            tar::EntryType::Continuous => Self::Continuous,
            tar::EntryType::GNULongName => Self::GNULongName,
            tar::EntryType::GNULongLink => Self::GNULongLink,
            tar::EntryType::GNUSparse => Self::GNUSparse,
            tar::EntryType::XGlobalHeader => Self::XGlobalHeader,
            tar::EntryType::XHeader => Self::XHeader,
            _ => Self::Other,
        }
    }
}
