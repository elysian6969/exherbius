use std::env::VarError;
use std::path::Path;
use std::{env, fmt, process};
use target::{Arch, Env, Sys, Target};

fn var_err(key: &str, error: VarError) -> ! {
    let msg = match error {
        VarError::NotPresent => format!("\x1b[38;5;1mexpected key: \x1b[38;5;2m{key}\x1b[m"),
        VarError::NotUnicode(_) => format!("\x1b[38;5;1mnot unicode: \x1b[38;5;2m{key}\x1b[m"),
    };

    eprintln!("paludis | {msg}");

    process::exit(1)
}

fn var(key: &str) -> String {
    let val = match env::var(key) {
        Ok(var) => var,
        Err(error) => var_err(key, error),
    };

    val
}

const DEFAULT_TARGET: Target = Target(Arch::X86_64, Sys::Linux, Env::Musl);

fn main() {
    eprintln!("paludis | \x1b[38;5;4msetting environment\x1b[m");

    let target = env::var("PALUDIS_CROSS_COMPILE_HOST")
        .ok()
        .and_then(|host| host.parse().ok())
        .unwrap_or(DEFAULT_TARGET);

    let category = var("CATEGORY");
    let package = var("PN");

    let mut flags = vec!["-Ofast", "-fomit-frame-pointer", "-march=native", "-pipe"];

    match (category.as_str(), package.as_str()) {
        ("dev-lang", "python") | ("sys-libs", "glibc") | ("sys-libs", "musl") => {
            flags.push("-fno-fast-math");
        }
        _ => {}
    }

    let usr = Path::new("/usr");
    let prefix = usr.join(target.as_gnu());

    let bin = prefix.join("bin");
    let lib = prefix.join("lib");

    let cmake = bin.join("cmake");

    let pkgconf = bin.join("pkgconf");
    let pkgconf_path = lib.join("pkgconfig");

    let locale = "en_US.UTF-8";

    let mut path = String::from("/milk/global:/usr/x86_64-pc-linux-musl/bin");

    if target != DEFAULT_TARGET {
        path.insert_str(0, &format!("{}:", bin.display()));
    }

    path.insert_str(0, "/usr/x86_64-pc-linux-musl/libexec/paludis/utils:/usr/x86_64-pc-linux-musl/libexec/paludis/utils/exheres-0:");

    export(&"CHOST", &DEFAULT_TARGET.as_gnu());
    export(&"CMAKE", &cmake);
    export(&"LANG", &locale);
    export(&"LC_ALL", &locale);
    export(&"PATH", &path);
    export(&"PKG_CONFIG", &pkgconf);
    export(&"PKG_CONFIG_PATH", &pkgconf_path);

    let flags = flags.join(" ");

    export_trusted(&"i686_pc_linux_gnu_CFLAGS", &flags);
    export_trusted(&"i686_pc_linux_gnu_CXXFLAGS", &flags);

    export_trusted(&"x86_64_pc_linux_gnu_CFLAGS", &flags);
    export_trusted(&"x86_64_pc_linux_gnu_CXXFLAGS", &flags);

    export_trusted(&"x86_64_pc_linux_musl_CFLAGS", &flags);
    export_trusted(&"x86_64_pc_linux_musl_CXXFLAGS", &flags);
}

fn export<K, V>(key: &K, val: &V)
where
    K: fmt::Display,
    V: fmt::Debug,
{
    eprintln!("paludis | \x1b[38;5;5m{key}\x1b[m=\x1b[38;5;2m{val:?}\x1b[m");
    print!("export {key}={val:?};");
}

fn export_trusted<K, V>(key: &K, val: &V)
where
    K: fmt::Display,
    V: fmt::Display,
{
    eprintln!("paludis | \x1b[38;5;5m{key}\x1b[m=\x1b[38;5;2m\"{val}\"\x1b[m");
    print!("export {key}=\"{val}\";");
}
