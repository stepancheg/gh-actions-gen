use crate::actions::checkout_sources_depth;
use crate::ghwf::Env;
use crate::ghwf::Job;
use crate::ghwf::Step;

pub fn super_linter_job() -> Job {
    let mut steps = Vec::new();
    steps.push(checkout_sources_depth(Some(0)));
    steps.push(
        Step::uses("super-linter", "github/super-linter@v3")
            .env("VALIDATE_ALL_CODEBASE", "false")
            .env("DEFAULT_BRANCH", "master")
            .env("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}")
            // Too many false positives
            .env("VALIDATE_JSCPD", "false")
            // Too many dull reports like how we should pluralise variable names
            .env("VALIDATE_PROTOBUF", "false"),
    );
    Job {
        id: "super-linter".to_owned(),
        name: "super-linter".to_owned(),
        runs_on: Env::UbuntuLatest,
        steps,
        ..Default::default()
    }
}
