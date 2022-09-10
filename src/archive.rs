#[cfg(feature = "tar")]
use std::fs::File;
#[cfg(feature = "zstd")]
use std::io::BufReader;
use std::path::Path;

use path_utils::PathExt;

#[cfg(feature = "bzip2")]
use bzip2::read::BzDecoder as BzSysDecoder;
#[cfg(feature = "bzip2-rs")]
use bzip2_rs::decoder::DecoderReader as BzNativeDecoder;
#[cfg(feature = "flate2")]
use flate2::read::GzDecoder;
#[cfg(feature = "lz4")]
use lz4::Decoder as Lz4Decoder;
#[cfg(feature = "tar")]
use tar::Archive as Tar;
#[cfg(feature = "xz2")]
use xz2::read::XzDecoder;
#[cfg(feature = "zstd")]
use zstd::stream::read::Decoder as ZstdDecoder;

use crate::Result;

/// Archive file.
// NONEXHAUSTIVE new formats could add new types
#[non_exhaustive]
// ALLOW constructing and storing Self is not the bottleneck, iterating the
// entries and I/O is
#[allow(clippy::large_enum_variant)]
pub enum Archive {
    #[cfg(feature = "tar")]
    #[doc(hidden)]
    Tar(Tar<File>),

    #[cfg(all(feature = "bzip2", feature = "tar"))]
    #[doc(hidden)]
    TarBzip2(Tar<BzSysDecoder<File>>),

    #[cfg(all(feature = "bzip2-rs", feature = "tar"))]
    #[doc(hidden)]
    TarBzip2Rs(Tar<BzNativeDecoder<File>>),

    #[cfg(all(feature = "flate2", feature = "tar"))]
    #[doc(hidden)]
    TarGzip(Tar<GzDecoder<File>>),

    #[cfg(all(feature = "lz4", feature = "tar"))]
    #[doc(hidden)]
    TarLz4(Tar<Lz4Decoder<File>>),

    #[cfg(all(feature = "xz2", feature = "tar"))]
    #[doc(hidden)]
    TarXz(Tar<XzDecoder<File>>),

    #[cfg(all(feature = "zstd", feature = "tar"))]
    #[doc(hidden)]
    TarZstd(Tar<ZstdDecoder<'static, BufReader<File>>>),
}

impl Archive {
    /// Returns an opened archive file.
    ///
    /// # Errors
    ///
    /// Returns an error if archive is of unsupported format or if opening it
    /// fails.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let extensions: Vec<String> = path.extensions_lossy().collect();
        let extensions: Vec<&str> =
            extensions.iter().map(String::as_str).collect();

        match extensions.as_slice() {
            #[cfg(all(feature = "bzip2", feature = "tar"))]
            ["tbz" | "tbz2", ..] | ["bz2", "tar", ..] => {
                let file = File::open(path)?;
                let file = BzSysDecoder::new(file);
                Ok(Self::TarBzip2(Tar::new(file)))
            }

            #[cfg(all(feature = "bzip2-rs", feature = "tar"))]
            ["tbz" | "tbz2", ..] | ["bz2", "tar", ..] => {
                let file = File::open(path)?;
                let file = BzNativeDecoder::new(file);
                Ok(Self::TarBzip2Rs(Tar::new(file)))
            }

            #[cfg(all(feature = "flate2", feature = "tar"))]
            ["tgz", ..] | ["gz", "tar", ..] => {
                let file = File::open(path)?;
                let file = GzDecoder::new(file);
                Ok(Self::TarGzip(Tar::new(file)))
            }

            #[cfg(all(feature = "lz4", feature = "tar"))]
            ["lz4", "tar", ..] => {
                let file = File::open(path)?;
                let file = Lz4Decoder::new(file)?;
                Ok(Self::TarLz4(Tar::new(file)))
            }

            #[cfg(all(feature = "xz2", feature = "tar"))]
            ["txz", ..] | ["xz", "tar", ..] => {
                let file = File::open(path)?;
                let file = XzDecoder::new(file);
                Ok(Self::TarXz(Tar::new(file)))
            }

            #[cfg(all(feature = "zstd", feature = "tar"))]
            ["zst", "tar", ..] => {
                let file = File::open(path)?;
                let file = ZstdDecoder::new(file)?;
                Ok(Self::TarZstd(Tar::new(file)))
            }

            #[cfg(feature = "tar")]
            ["tar", ..] => {
                let file = File::open(path)?;
                Ok(Self::Tar(Tar::new(file)))
            }

            _ => Err(crate::Error::UnsupportedArchiveType(path.to_path_buf())),
        }
    }
}
