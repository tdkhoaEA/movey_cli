use std::{collections::HashMap, fs};

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use toml_edit::easy::{self, Value};

#[derive(Deserialize, Debug)]
pub struct Dependencies {
    pub dependencies: MoveyDependencies,
}

#[derive(Deserialize, Debug)]
pub struct MoveyDependencies {
    pub movey: MoveyResolver,
}

#[derive(Deserialize, Debug)]
pub struct MoveyResolver {
    pub resolver: String,
    pub packages: Option<HashMap<String, String>>,
}

pub struct MovePackageResolver {}

impl MovePackageResolver {
    pub fn execute() -> Result<()> {
        let move_toml_content = String::from_utf8(
            fs::read("Move.toml").context("Not in Move package root directory")?,
        )?;
        let deps = MovePackageResolver::parse_deps_from_toml(&move_toml_content)?;
        println!("{:#?}", deps);
        Ok(())
    }

    fn parse_deps_from_toml(toml_content: &str) -> Result<HashMap<String, String>> {
        let toml = easy::from_str::<Dependencies>(toml_content)
            .map_err(|e| anyhow!("Wrong Move.toml format for Movey dependencies. Error: {e}"))?;
        if toml.dependencies.movey.resolver != "movey" {
            bail!("The CLI only resolve Movey-style dependencies.")
        };

        let mut res = HashMap::new();

        if let Some(offchain_deps) = toml.dependencies.movey.packages {
            for (key, value) in offchain_deps {
                res.insert(key, value);
            }
        };
        return Ok(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_deps_from_toml() {
        let toml_content = r#"
            [package]
            name = ""
            version = ""

            [dependencies]
            
            [dependencies.movey]
            resolver = "movey"
            
            [dependencies.movey.packages]
            movedemo-ea = "1.0.0"
            movedemo-ea-02 = "0.5.0"
        "#;
        let result = MovePackageResolver::parse_deps_from_toml(toml_content).unwrap();
        assert!(result.contains_key("movedemo-ea"));

        let sui_deps = result.get("movedemo-ea").unwrap();
        assert!(sui_deps.contains("1.0.0"))
    }
}
