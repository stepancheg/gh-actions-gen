use crate::cache::cache_step;
use crate::ghwf::Step;
use crate::yaml::Yaml;
use std::fmt;

pub fn checkout_sources_depth(depth: Option<u32>) -> Step {
    let mut step = Step::uses("Checkout sources", "actions/checkout@v2");
    if let Some(depth) = depth {
        step.with = Some(Yaml::Map(vec![(
            "fetch-depth".to_owned(),
            Yaml::Int(depth as _),
        )]));
    }
    step
}

pub fn checkout_sources() -> Step {
    checkout_sources_depth(None)
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum RustToolchain {
    Stable,
    Beta,
    Nightly,
}

impl fmt::Display for RustToolchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustToolchain::Stable => write!(f, "stable"),
            RustToolchain::Beta => write!(f, "beta"),
            RustToolchain::Nightly => write!(f, "nightly"),
        }
    }
}

pub fn rust_install_toolchain(channel: RustToolchain) -> Step {
    Step::uses_with(
        "Install toolchain",
        "actions-rs/toolchain@v1",
        Yaml::map(vec![
            ("profile", Yaml::from("minimal")),
            ("toolchain", Yaml::from(format!("{}", channel))),
            ("override", Yaml::from(true)),
        ]),
    )
}

pub fn cargo(name: &str, command: &str, args: &str) -> Step {
    let mut with = vec![("command", command)];
    if !args.is_empty() {
        with.push(("args", args));
    }
    Step::uses_with(name, "actions-rs/cargo@v1", Yaml::map(with))
}

pub fn cargo_test(name: &str, args: &str) -> Step {
    cargo(name, "test", args)
}

pub fn cargo_build(name: &str, args: &str) -> Step {
    cargo(name, "build", args)
}

pub fn cargo_doc(name: &str, args: &str) -> Step {
    cargo(name, "doc", args)
}

pub fn cargo_cache() -> Step {
    // https://github.com/actions/cache/blob/main/examples.md#rust---cargo
    // TODO: also cache `target` directory
    cache_step(
        "cargo cache",
        "${{ runner.os }}-cargo-2",
        &["~/.cargo/registry", "~/.cargo/git"],
    )
}
