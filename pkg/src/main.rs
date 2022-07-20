use clap::Parser;
use mix_triple::{Arch, Env, Triple};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::process::Command;

const DEFAULT_TARGET: Target = Target(Arch::X86_64, Sys::Linux, Env::Musl);

const DB: &str = "/var/db/paludis/repositories/installed/data";
const I686_DB: &str = "/var/db/paludis/repositories/cross-installed/i686-pc-linux-gnu/data";
const X86_64_DB: &str = "/var/db/paludis/repositories/cross-installed/x86_64-pc-linux-gnu/data";

fn read_db(db: &str) -> BTreeSet<String> {
    let mut packages = BTreeSet::new();

    for entry in fs::read_dir(db).expect("read database") {
        let entry = entry.expect("read database entry");
        let path = entry.path();
        let name = path.file_name().expect("file name");
        let name = name.to_str().expect("file name");
        let name = name.replace("---", "/");

        packages.insert(name);
    }

    packages
}

fn cave() -> Command {
    let mut command = Command::new("cave");

    command
        .current_dir("/var/tmp/paludis")
        .env_clear()
        .env("HOME", "/var/tmp/paludis")
        .env("PALUDIS_DO_NOTHING_SANDBOXY", "1")
        .env("LANG", "en_US.UTF-8")
        .env("LC_ALL", "en_US.UTF-8")
        .env("PATH", "/milk/global:/usr/x86_64-pc-linux-musl/bin")
        .env("TERM", "xterm-256color");

    command
}

fn resolve(target: Target) -> Command {
    let mut command = cave();

    if target.is_gnu() {
        command.args(["--environment", ":gnu"]);
    }

    command.arg("resolve");

    if target.is_gnu() {
        command
            .args(["--cross-host", "i686-pc-linux-gnu"])
            .args(["--make", "cross-compile"]);
    }

    if target.is_musl() {
        command
            .args(["--no-blockers-from", "dev-libs/argp-standalone"])
            .args(["--no-blockers-from", "dev-libs/musl-fts"])
            .args(["--no-blockers-from", "dev-libs/musl-obstack"])
            .args(["--no-blockers-from", "sys-libs/musl-compat"]);
    }

    command
}

fn add(add: Add) -> io::Result<()> {
    let command = cave::resolve(add.target);

    command
        .arg("--preserve-world")
        .args(packages)
        .arg("--execute");

    let mut child = command.spawn()?;
    let _wait = child.wait()?;

    Ok(())
}

fn update(update: Update) -> io::Result<()> {
    let command = cave::resolve(add.target);

    command
        .arg("--complete")
        .args(["--continue-on-failure", "if-satisfied"])
        .args(["--keep", "if-same-metadata"])
        .args(["--keep-targets", "if-same-metadata"])
        .arg("world")
        .arg("--execute");

    let mut child = command.spawn()?;
    let _wait = child.wait()?;

    Ok(())
}

fn sync() {
    let mut command = cave();

    command.arg("sync");

    println!("{command:?}");

    let mut child = command.spawn().expect("cave sync");
    let _wait = child.wait();
}

fn info() {
    let main = read_db(DB);
    let i686 = read_db(I686_DB);
    let x86_64 = read_db(X86_64_DB);

    let x86_64_musl = main.len();
    let i686_gnu = i686.len();
    let x86_64_gnu = x86_64.len();
    let total = x86_64_musl + i686_gnu + x86_64_gnu;

    println!("{total} (x86_64-musl {x86_64_musl}, i686-gnu {i686_gnu}, x86_64-gnu {x86_64_gnu})");
}

use options::Options;

mod options;

fn main() {
    let options = Options::parse();

    match options {
        Options::Add(add) => resolve(add.list, add.triple),
        Options::Info => info(),
        Options::Remove(remove) => resolve(
            remove.list.into_iter().map(|pkg| format!("!{pkg}")),
            remove.triple,
        ),
        Options::Sync => sync(),
        Options::Update(info) => update(info.triple),
    }
}
