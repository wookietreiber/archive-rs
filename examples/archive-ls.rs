use std::path::{Path, PathBuf};

use anyhow::Result;
use archive_rs::{Archive, Entry};
use bytesize::ByteSize;
use clap::{Arg, ArgAction, ArgMatches, Command};

#[derive(Copy, Clone)]
struct Config {
    pub long: bool,
    pub humanize: bool,
}

fn main() -> Result<()> {
    let args = args();

    let config = Config {
        long: args.get_flag("long"),
        humanize: args.get_flag("humanize"),
    };

    let archives: Vec<&PathBuf> = args
        .get_many("archive")
        .expect("`archive`s are required")
        .collect();

    for path in archives {
        let mut archive = Archive::open(path)?;

        for entry in archive.entries()? {
            let entry = entry?;
            ls_entry(path, &entry, config)?;
        }
    }

    Ok(())
}

fn ls_entry(archive: &Path, entry: &Entry, config: Config) -> Result<()> {
    let payload = if config.long {
        let size = if config.humanize {
            humanized(entry.size())
        } else {
            entry.size().to_string()
        };

        format!(" {}", size)
    } else {
        String::from("")
    };

    println!(
        "{} {}{}",
        archive.display(),
        entry.path()?.display(),
        payload
    );

    Ok(())
}

fn humanized(bytes: u64) -> String {
    ByteSize(bytes)
        .to_string_as(true)
        .replace("iB", "")
        .replace(['B', ' '], "")
        .to_uppercase()
}

fn args() -> ArgMatches {
    cli().get_matches()
}

fn cli() -> Command {
    let archive = Arg::new("archive")
        .action(ArgAction::Append)
        .value_parser(clap::value_parser!(PathBuf))
        .help("archive files");

    let humanize = Arg::new("humanize")
        .short('h')
        .long("humanize")
        .action(ArgAction::SetTrue)
        .help("humanize bytes");

    let long = Arg::new("long")
        .short('l')
        .long("long")
        .action(ArgAction::SetTrue)
        .help("print extended metadata");

    let help = Arg::new("help")
        .short('?')
        .long("help")
        .action(ArgAction::Help)
        .help("print help (use --help to see all options)")
        .long_help("Print help.");

    Command::new("archive-ls")
        .arg(archive)
        .arg(humanize)
        .arg(long)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(help)
}
