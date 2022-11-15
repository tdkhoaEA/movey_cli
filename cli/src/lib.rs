use clap::{App, AppSettings};
use clap::{crate_version, crate_description, crate_authors};

use core::commands;
use utils::error::Result;

pub mod base;
use base::movey_login::MoveyLogin;
use base::movey_upload::MoveyUpload;

/// Match commands
pub fn cli_match() -> Result<()> {
    // Get matches
    let cli_matches = cli_config()?;

    // Merge clap config file if the value is set
    // AppConfig::merge_config(cli_matches.value_of("config"))?;

    // Matches Commands or display help
    match cli_matches.subcommand_name() {
        Some("hazard") => {
            commands::hazard()?;
        }
        Some("error") => {
            commands::simulate_error()?;
        }
        Some("config") => {
            commands::config()?;
        }
        Some("login") => {
            MoveyLogin::execute()?;
        },
        Some("upload") => {
            MoveyUpload::execute(None)?
        }
        _ => {
            // Arguments are required by default (in Clap)
            // This section should never execute and thus
            // should probably be logged in case it executed.
        }
    }
    Ok(())
}

/// Configure Clap
/// This function will configure clap and match arguments
pub fn cli_config() -> Result<clap::ArgMatches> {
    let cli_app = App::new("movey-cli")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .subcommand(App::new("login").about("Login to Movey"))
        .subcommand(App::new("upload").about("Upload to Movey"));
    // Get matches
    let cli_matches = cli_app.get_matches();

    Ok(cli_matches)
}
