use crate::actions::checkout_sources_depth;
use crate::ghwf::Env;
use crate::ghwf::Job;
use crate::ghwf::Step;

enum WhatLinter {
    SuperLinter,
    MegaLinter,
}

fn linter_job(linter: WhatLinter) -> Job {
    let (id, action) = match linter {
        WhatLinter::SuperLinter => ("super-linter", "github/super-linter@v4"),
        WhatLinter::MegaLinter => ("mega-linter", "megalinter/megalinter@v5"),
    };

    let mut steps = Vec::new();
    steps.push(checkout_sources_depth(Some(0)));
    steps.push(
        Step::uses(id, action)
            .env("VALIDATE_ALL_CODEBASE", "false")
            .env("DEFAULT_BRANCH", "master")
            .env("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}")
            // Too many false positives
            .env("VALIDATE_JSCPD", "false")
            .env("VALIDATE_COPYPASTE_JSCPD", "false")
            // Too many dull reports like how we should pluralise variable names
            .env("VALIDATE_PROTOBUF", "false")
            // These are buggy, they don't detect edition properly
            .env("VALIDATE_RUST_2015", "false")
            .env("VALIDATE_RUST_2018", "false")
            // Clippy is too noisy
            .env("VALIDATE_RUST_CLIPPY", "false"),
    );
    Job {
        id: id.to_owned(),
        name: id.to_owned(),
        runs_on: Env::UbuntuLatest,
        steps,
        ..Default::default()
    }
}

/// Job to run the [super-linter](https://github.com/github/super-linter).
pub fn super_linter_job() -> Job {
    linter_job(WhatLinter::SuperLinter)
}

/// Job to run the [mega-linter](https://github.com/megalinter/megalinter/).
pub fn mega_linter_job() -> Job {
    linter_job(WhatLinter::MegaLinter)
}
