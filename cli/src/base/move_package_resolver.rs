use std::{collections::HashMap, fs};

use anyhow::{anyhow, bail, Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use toml_edit::easy::{self, value::Table, Value};
use utils::{env::MOVE_HOME, movey_credential};

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

#[derive(Debug, Serialize)]
pub struct MovePackageResolver {
    deps_map: HashMap<String, String>,
    deps_resolved: HashMap<String, MoveDependency>,
}

impl MovePackageResolver {
    fn new() -> Self {
        MovePackageResolver {
            deps_map: HashMap::new(),
            deps_resolved: HashMap::new(),
        }
    }

    pub fn execute() -> Result<()> {
        let move_toml_content = String::from_utf8(
            fs::read("Move.toml").context("Not in Move package root directory")?,
        )?;

        let mut deps = MovePackageResolver::new();
        deps.parse_deps_from_toml(&move_toml_content)?;
        deps.resolve_movey_deps(None)?;
        let lock_file_content = deps.generate_lock_toml_content();

        fs::write("Move.lock", lock_file_content)
            .map_err(|e| anyhow!("Cannot write to Move.lock file. {e}"))?;
        Ok(())
    }

    fn parse_deps_from_toml(&mut self, toml_content: &str) -> Result<()> {
        let toml = easy::from_str::<Dependencies>(toml_content)
            .map_err(|e| anyhow!("Wrong Move.toml format for Movey dependencies. Error: {e}"))?;
        if toml.dependencies.movey.resolver != "movey" {
            bail!("The CLI only resolve Movey dependencies.")
        };
        if let Some(offchain_deps) = toml.dependencies.movey.packages {
            for (key, value) in offchain_deps {
                self.deps_map.insert(key, value);
            }
        };
        Ok(())
    }

    fn resolve_movey_deps(&mut self, url: Option<&str>) -> Result<()> {
        let url = match url {
            Some(url) => url.to_owned(),
            None => movey_credential::get_movey_url(&MOVE_HOME)?,
        };
        let schemes = self.deps_map.values().collect::<Vec<_>>();
        let schemes_request = json!({
            "schemes": schemes,
        });
        let client = Client::new();
        let response = client
            .post(&format!("{}/api/v1/packages/info", &url))
            .json(&schemes_request)
            .send();
        match response {
            Ok(response) => {
                self.deps_resolved = response.json()?;
                Ok(())
            }
            Err(_) => {
                bail!("An unexpected error occurred. Please try again later");
            }
        }
    }

    fn generate_lock_toml_content(&self) -> String {
        let mut lock_toml = Value::Table(Table::new());
        let packages = Value::Array(
            self.deps_resolved
                .values()
                .map(|e| Value::from(e.clone()))
                .collect::<Vec<_>>(),
        );
        lock_toml
            .as_table_mut()
            .unwrap()
            .insert(String::from("package"), packages);
        lock_toml.to_string()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MoveDependency {
    name: String,
    version: String,
    repository_url: String,
    rev: String,
    scheme: String,
}

impl From<MoveDependency> for Value {
    fn from(dep: MoveDependency) -> Self {
        let mut package = Table::new();
        package.insert(String::from("name"), Value::String(dep.name));
        package.insert(String::from("version"), Value::String(dep.version));
        package.insert(String::from("repository_url"), Value::String(dep.repository_url));
        package.insert(String::from("rev"), Value::String(dep.rev));
        Value::Table(package)
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
            MoveDemo = "movedemo-ea:1.0.0"
            MoveDemo2 = "movedemo-ea-02:0.5.0"
        "#;
        let mut result = MovePackageResolver::new();
        result.parse_deps_from_toml(toml_content).unwrap();
        assert!(result.deps_map.contains_key("MoveDemo"));

        let sui_deps = result.deps_map.get("MoveDemo").unwrap();
        assert!(sui_deps.contains("movedemo-ea:1.0.0"))
    }

    #[test]
    fn generate_lock_toml_content_works() {
        let mut resolver = MovePackageResolver::new();
        let mut mock_deps_resolved = HashMap::new();
        mock_deps_resolved.insert(
            "movedemo-ea".to_owned(),
            MoveDependency {
                name: "movedemo".to_owned(),
                version: "1.0.0".to_owned(),
                repository_url: "repo_url".to_owned(),
                rev: "rev1".to_owned(),
                scheme: "movedemo-ea".to_owned(),
            },
        );
        mock_deps_resolved.insert(
            "movedemo-ea-02".to_owned(),
            MoveDependency {
                name: "movedemo".to_owned(),
                version: "0.5.0".to_owned(),
                repository_url: "repo_url_02".to_owned(),
                rev: "rev2".to_owned(),
                scheme: "movedemo-ea-02".to_owned(),
            },
        );
        resolver.deps_resolved = mock_deps_resolved;
        let lock_toml_content = resolver.generate_lock_toml_content();
        let expected_1 = r#"[[package]]
name = "movedemo"
version = "1.0.0"
repository_url = "repo_url"
rev = "rev1"
"#;
        let expected_2 = r#"[[package]]
name = "movedemo"
version = "0.5.0"
repository_url = "repo_url_02"
rev = "rev2"
"#;
        // Because HashMap iterator does not preserve order
        assert!(lock_toml_content.contains(expected_1));
        assert!(lock_toml_content.contains(expected_2));
    }
}
