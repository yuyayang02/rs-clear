use clap::Parser;
use inquire::MultiSelect;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

/// find and delete `target/` directories next to `Cargo.toml`
#[derive(Parser, Debug)]
#[command(author, version, about, styles = CLAP_STYLING)]
struct Args {
    /// root path to search (optional, default is current directory)
    #[arg(value_name = "PATH", default_value = ".")]
    path: PathBuf,

    /// delete all target directories without prompting
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = Args::parse();

    println!("scanning directory: {}", args.path.display());

    const CARGO_CONFIG_FILE: &'static str = "Cargo.toml";
    const RUST_CACHE_DIR: &'static str = "target";

    let mut candidates = vec![];

    for entry in WalkDir::new(&args.path)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name() == CARGO_CONFIG_FILE)
    {
        if let Some(parent) = entry.path().parent() {
            let target = parent.join(RUST_CACHE_DIR);
            if target.exists() {
                candidates.push(target);
            }
        }
    }

    if candidates.is_empty() {
        println!("no target directories found");
        return;
    }

    let to_delete: Vec<PathBuf> = if args.all {
        candidates
    } else {
        let options: Vec<String> = candidates.iter().map(|p| p.display().to_string()).collect();

        let defaults: Vec<usize> = (0..options.len()).collect();

        match MultiSelect::new("select target directories to delete:", options)
            .with_default(&defaults)
            .prompt()
        {
            Ok(selected) => selected.into_iter().map(PathBuf::from).collect(),
            Err(err) => {
                eprintln!("prompt cancelled or failed: {}", err);
                return;
            }
        }
    };

    for path in to_delete {
        print!("deleting: {}\r", path.display());
        io::stdout().flush().unwrap();

        match fs::remove_dir_all(&path) {
            Ok(_) => println!("[deleted] {}", path.display()),
            Err(e) => println!("[failed]  {}: {}", path.display(), e),
        }
    }

    println!("done");
}
