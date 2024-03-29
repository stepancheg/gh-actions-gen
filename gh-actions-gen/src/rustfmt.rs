use crate::actions::checkout_sources;
use crate::actions::rust_install_toolchain;
use crate::actions::RustToolchain;
use crate::ghwf::Job;
use crate::ghwf::Step;

pub fn rustfmt_check_step() -> Step {
    Step::run("cargo fmt check", "cargo fmt -- --check")
}

pub fn rustfmt_check_job() -> Job {
    let mut steps = Vec::new();
    steps.push(checkout_sources());
    steps.push(rust_install_toolchain(RustToolchain::Stable));
    steps.push(Step::run("print version", "cargo fmt --version"));
    steps.push(rustfmt_check_step());
    Job {
        id: "rustfmt-check".to_owned(),
        name: "rustfmt check".to_owned(),
        steps,
        ..Default::default()
    }
}
