// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0
pub mod movey_login;
pub mod movey_upload;

// use move_package::source_package::layout::SourcePackageLayout;
// use std::path::PathBuf;

// pub fn reroot_path(path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
//     let path = path.unwrap_or_else(|| PathBuf::from("."));
//     // Always root ourselves to the package root, and then compile relative to that.
//     let rooted_path = SourcePackageLayout::try_find_root(&path.canonicalize()?)?;
//     std::env::set_current_dir(&rooted_path).unwrap();

//     Ok(PathBuf::from("."))
// }
