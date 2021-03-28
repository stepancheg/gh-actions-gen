use std::fs::File;
use std::io::Write;

use crate::ghwf::Job;
use crate::yaml::Yaml;
use crate::yaml::YamlWriter;

pub mod actions;
pub mod ghwf;
pub mod yaml;

pub fn write(jobs: Vec<Job>) {
    let jobs = Yaml::map(jobs.into_iter().map(Job::into));

    let yaml = Yaml::map(vec![
        ("on", Yaml::list(vec!["push", "pull_request"])),
        ("name", Yaml::string("Jobs")),
        ("jobs", jobs),
    ]);

    let mut writer = YamlWriter::default();
    writer.write_line(&format!(
        "# @generated by {}, do not edit",
        env!("CARGO_PKG_NAME")
    ));
    writer.write_line("");
    writer.write_yaml(&yaml);
    File::create(".github/workflows/ci.yml")
        .unwrap()
        .write_all(writer.buffer.as_bytes())
        .unwrap();
}
