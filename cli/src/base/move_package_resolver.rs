use std::{collections::HashMap, fs};

use anyhow::{bail, Context, Result};
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
    pub packages: Option<HashMap<String, Value>>,
    pub onchain: Option<HashMap<String, Value>>,
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

    fn parse_deps_from_toml(toml_content: &str) -> Result<HashMap<String, Value>> {
        let toml = easy::from_str::<Dependencies>(toml_content)
            .map_err(|e| bail!("Wrong Move.toml format for Movey dependencies. Error: {e}"))?;
        if toml.dependencies.movey.resolver != "movey" {
            bail!("The CLI only resolve Movey-style dependencies.")
        };

        let mut res = HashMap::new();

        if let Some(offchain_deps) = toml.dependencies.movey.packages {
            for (key, value) in offchain_deps {
                res.insert(key, value);
            }
        };
        if let Some(onchain_deps) = toml.dependencies.movey.onchain {
            for (key, value) in onchain_deps {
                res.insert(key, value);
            }
        }
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
            Sui  = { ns = "sui/stdlib", version = "1.0.0" }
            Move = "move-language/stdlib"

            [dependencies.movey.onchain]
            coin = { addr = "0x2", chain = "sui/devnet" }
            aptos-network-core = "aptos/mainnet"
        "#;
        let result = MovePackageResolver::parse_deps_from_toml(toml_content).unwrap();
        assert!(result.contains_key("Sui"));

        let sui_deps = result.get("Sui").unwrap();
        assert_eq!(sui_deps.as_table().unwrap().len(), 2)
    }
}
