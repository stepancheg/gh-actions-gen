use crate::ghwf::Step;
use crate::yaml::Yaml;

pub fn cache_step(name: &str, key: &str, path: &[&str]) -> Step {
    Step::uses_with(
        name,
        "actions/cache@v2",
        Yaml::map(vec![
            (
                "path",
                Yaml::string(path.iter().map(|p| format!("{}\n", p)).collect::<String>()),
            ),
            ("key", Yaml::string(key)),
        ]),
    )
}
