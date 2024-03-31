//! Which formats and file endings this library supports, depends on cargo
//! features.

use std::fmt;

/// Supported formats.
#[derive(Clone, Copy, Debug)]
pub enum Format {
    #[cfg(feature = "tar")]
    /// tarball
    Tar,

    #[cfg(all(
        feature = "tar",
        any(feature = "bzip2", feature = "bzip2-rs")
    ))]
    /// bzip2-compressed tarball
    TarBzip2,

    #[cfg(all(feature = "tar", feature = "flate2"))]
    /// gzip-compressed tarball
    TarGzip,

    #[cfg(all(feature = "tar", feature = "lz4"))]
    /// lz4-compressed tarball
    TarLz4,

    #[cfg(all(feature = "tar", feature = "xz2"))]
    /// xz-compressed tarball
    TarXz,

    #[cfg(all(feature = "tar", feature = "zstd"))]
    /// zstd-compressed tarball
    TarZstd,
}

impl Format {
    /// Returns the format name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "tar")]
            Self::Tar => "Tar",

            #[cfg(all(
                feature = "tar",
                any(feature = "bzip2", feature = "bzip2-rs")
            ))]
            Self::TarBzip2 => "TarBzip2",

            #[cfg(all(feature = "tar", feature = "flate2"))]
            Self::TarGzip => "TarGz",

            #[cfg(all(feature = "tar", feature = "lz4"))]
            Self::TarLz4 => "TarLz4",

            #[cfg(all(feature = "tar", feature = "xz2"))]
            Self::TarXz => "TarXz",

            #[cfg(all(feature = "tar", feature = "zstd"))]
            Self::TarZstd => "TarZstd",
        }
    }

    /// Returns the format description, like *gzip-compressed tarball*.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            #[cfg(feature = "tar")]
            Self::Tar => "tarball",

            #[cfg(all(
                feature = "tar",
                any(feature = "bzip2", feature = "bzip2-rs")
            ))]
            Self::TarBzip2 => "bzip2-compressed tarball",

            #[cfg(all(feature = "tar", feature = "flate2"))]
            Self::TarGzip => "gzip-compressed tarball",

            #[cfg(all(feature = "tar", feature = "lz4"))]
            Self::TarLz4 => "lz4-compressed tarball",

            #[cfg(all(feature = "tar", feature = "xz2"))]
            Self::TarXz => "xz-compressed tarball",

            #[cfg(all(feature = "tar", feature = "zstd"))]
            Self::TarZstd => "zstd-compressed tarball",
        }
    }

    /// Returns the supported file endings for format auto-detection.
    #[must_use]
    pub fn file_endings(&self) -> Vec<&'static str> {
        match self {
            #[cfg(feature = "tar")]
            Self::Tar => vec!["*.tar"],

            #[cfg(all(
                feature = "tar",
                any(feature = "bzip2", feature = "bzip2-rs")
            ))]
            Self::TarBzip2 => {
                vec!["*.tar.bz2", "*.tbz", "*.tbz2"]
            }

            #[cfg(all(feature = "tar", feature = "flate2"))]
            Self::TarGzip => vec!["*.tar.gz", "*.tgz"],

            #[cfg(all(feature = "tar", feature = "lz4"))]
            Self::TarLz4 => vec!["*.tar.lz4"],

            #[cfg(all(feature = "tar", feature = "xz2"))]
            Self::TarXz => vec!["*.tar.xz", "*.txz"],

            #[cfg(all(feature = "tar", feature = "zstd"))]
            Self::TarZstd => vec!["*.tar.zst"],
        }
    }

    /// Returns all supported format names.
    #[must_use]
    pub const fn all_names<'a>() -> &'a [Self] {
        &[
            #[cfg(feature = "tar")]
            Self::Tar,
            #[cfg(all(
                feature = "tar",
                any(feature = "bzip2", feature = "bzip2-rs")
            ))]
            Self::TarBzip2,
            #[cfg(all(feature = "tar", feature = "flate2"))]
            Self::TarGzip,
            #[cfg(all(feature = "tar", feature = "lz4"))]
            Self::TarLz4,
            #[cfg(all(feature = "tar", feature = "xz2"))]
            Self::TarXz,
            #[cfg(all(feature = "tar", feature = "zstd"))]
            Self::TarZstd,
        ]
    }

    /// Returns all pre-formatted formats in the form of `name description
    /// [endings..]`.
    #[must_use]
    pub fn all_file_endings() -> Vec<String> {
        vec![
            #[cfg(feature = "tar")]
            Self::Tar.describe(),
            #[cfg(all(
                feature = "tar",
                any(feature = "bzip2", feature = "bzip2-rs")
            ))]
            Self::TarBzip2.describe(),
            #[cfg(all(feature = "tar", feature = "flate2"))]
            Self::TarGzip.describe(),
            #[cfg(all(feature = "tar", feature = "lz4"))]
            Self::TarLz4.describe(),
            #[cfg(all(feature = "tar", feature = "xz2"))]
            Self::TarXz.describe(),
            #[cfg(all(feature = "tar", feature = "zstd"))]
            Self::TarZstd.describe(),
        ]
    }

    fn describe(self) -> String {
        format!(
            "{} {} {:?}",
            self.name(),
            self.description(),
            self.file_endings()
        )
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
