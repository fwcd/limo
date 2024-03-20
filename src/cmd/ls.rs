use std::fmt;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use crate::{context::Context, path::VirtualPathBuf};

#[derive(Parser)]
#[command(bin_name = "ls")]
struct Args {
    #[arg(short, long, action, help = "Include . and ..")]
    all: bool,

    #[arg(short, long, action, help = "Use a long listing format")]
    long: bool,

    #[arg(default_value = ".", help = "The directory to list")]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);
    let response = ctx.lh.list(&path.as_lh_vec()).await?;
    let mut entries: Vec<Entry> = response.payload.entries
        .into_iter()
        .map(|(name, contents)| Entry { name, is_directory: contents.is_some() })
        .collect();

    if args.all {
        entries.extend([".", ".."].map(|s| Entry { name: s.to_string(), is_directory: true }));
    }

    entries.sort();

    if args.long {
        // TODO: Print more metadata
        for entry in entries {
            println!("{}", entry);
        }
    } else {
        println!("{}", entries.into_iter().map(|e| format!("{}", e)).collect::<Vec<_>>().join("   "));
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    name: String,
    is_directory: bool,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_directory {
            write!(f, "{}", self.name.blue())
        } else {
            write!(f, "{}", self.name)
        }
    }
}
