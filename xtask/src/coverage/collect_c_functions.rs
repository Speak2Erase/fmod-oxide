// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use indexmap::{IndexMap, IndexSet};

pub type CategorySet = IndexSet<String>;
pub type FunctionMap = IndexMap<String, (usize, bool)>;

const WRAPPER_H_PATH: &str = "fmod-sys/src/wrapper.h";

pub fn collect(
    core_include_dir: PathBuf,
    studio_include_dir: PathBuf,
    verbose: bool,
) -> color_eyre::Result<(CategorySet, FunctionMap)> {
    let clang = clang::Clang::new().unwrap();

    let index = clang::Index::new(&clang, false, true);
    let translation_unit = index
        .parser(WRAPPER_H_PATH)
        .arguments(&[
            "-I",
            core_include_dir.to_str().unwrap(),
            "-I",
            studio_include_dir.to_str().unwrap(),
        ])
        .parse()?;
    let entities = translation_unit.get_entity().get_children();

    let category_regex = regex::Regex::new(r"FMOD_(Studio_)?([A-Za-z0-9]*)_.*$")?;

    let mut categories = IndexSet::new();
    let mut c_functions: FunctionMap = clang::sonar::find_functions(entities)
        .filter(|f| f.entity.get_linkage().unwrap() != clang::Linkage::Internal)
        .map(|f| {
            let category = category_regex
                .captures(&f.name)
                .map(|c| {
                    if c.get(1).is_some() {
                        format!("Studio {}", c.get(2).unwrap().as_str())
                    } else {
                        c.get(2).unwrap().as_str().to_string()
                    }
                })
                .unwrap_or_else(|| "Unknown".to_string());

            if verbose {
                println!("Found C function: {}: {}", f.name, category);
            }

            let (category, _) = categories.insert_full(category);

            (f.name, (category, false))
        })
        .collect();
    c_functions.sort_by(|_, (c1, _), _, (c2, _)| c1.cmp(c2));

    Ok((categories, c_functions))
}
