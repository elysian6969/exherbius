use clap::Parser;
use target::{Arch, Env, Sys, Triple};

const DEFAULT_TARGET: Target = Target(Arch::X86_64, Sys::Linux, Env::Musl);

#[derive(Parser)]
pub struct Add {
    #[clap(default_value = DEFAULT_TARGET, long)]
    pub target: Target,
    pub list: Vec<String>,
}

#[derive(Parser)]
pub struct Remove {
    #[clap(default_value = DEFAULT_TARGET, long)]
    pub target: Target,
    pub list: Vec<String>,
}

#[derive(Parser)]
pub struct Update {
    #[clap(default_value = DEFAULT_TARGET, long)]
    pub target: Target,
}

#[derive(Parser)]
pub enum Options {
    Add(Add),
    Info,
    Remove(Remove),
    Sync,
    Update(Update),
}

impl Options {
    #[inline]
    pub fn parse() -> Self {
        <Options as Parser>::parse()
    }
}
