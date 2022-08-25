use std::path::{Path, PathBuf};

use anyhow::Result;
use archive_rs::{Archive, Entry};
use bytesize::ByteSize;
use clap::{Arg, ArgMatches, Command};

#[derive(Copy, Clone)]
struct Config {
    pub long: bool,
    pub humanize: bool,
}

fn main() -> Result<()> {
    let args = args();

    let archives: Vec<&PathBuf> = args
        .get_many("archive")
        .expect("`archive`s are required")
        .collect();

    let config = Config {
        long: args.contains_id("long"),
        humanize: args.contains_id("humanize"),
    };

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
        .replace('B', "")
        .replace(' ', "")
        .to_uppercase()
}

fn args() -> ArgMatches {
    cli().get_matches()
}

fn cli() -> Command<'static> {
    let archive = Arg::new("archive")
        .required(true)
        .multiple_values(true)
        .value_parser(clap::value_parser!(PathBuf))
        .help("archive files");

    let humanize = Arg::new("humanize")
        .short('h')
        .long("humanize")
        .help("humanize bytes");

    let long = Arg::new("long")
        .short('l')
        .long("long")
        .help("print extended metadata");

    Command::new("archive-ls")
        .arg(archive)
        .arg(humanize)
        .arg(long)
        .mut_arg("help", |a| a.short('?'))
}
