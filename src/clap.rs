//! Integration with [clap](https://docs.rs/clap).

use clap::builder::PossibleValue;
use clap::{Arg, ArgAction, ValueEnum};

/// Returns a [`clap::Arg`].
///
/// # Example
///
/// ```
/// use clap::{Arg, Command};
///
/// let cli = Command::new("mytool")
///     .arg(archive_rs::clap::list_archive_formats());
///
/// let args = cli.get_matches_from(vec![
///     "mytool", "--list-archive-formats",
/// ]);
///
/// if args.get_flag("list-archive-formats") {
///     for x in archive_rs::support::Format::all_file_endings() {
///         println!("{}", x);
///     }
/// }
/// ```
#[must_use]
pub fn list_archive_formats() -> Arg {
    Arg::new("list-archive-formats")
        .long("list-archive-formats")
        .action(ArgAction::SetTrue)
        .help("list supported archive formats")
}

impl ValueEnum for crate::support::Format {
    fn value_variants<'a>() -> &'a [Self] {
        Self::all_names()
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.name()))
    }
}
