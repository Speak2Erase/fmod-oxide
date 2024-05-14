// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io::Write;
use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;

mod collect_c_functions;
mod mark_rust_functions;

pub fn coverage(
    core_include_dir: PathBuf,
    studio_include_dir: PathBuf,
    print: bool,
    verbose: bool,
) -> color_eyre::Result<()> {
    let (categories, mut c_functions) =
        collect_c_functions::collect(core_include_dir, studio_include_dir, verbose)?;

    mark_rust_functions::mark(&mut c_functions, verbose)?;

    let mut coverage_md = std::fs::File::create("COVERAGE.md")?;
    let channel_filter_regex = regex::Regex::new(r"FMOD_(Channel|ChannelGroup)_(.*)$")?;
    let mut current_category = usize::MAX;

    for (name, &(category, exists)) in c_functions.iter().filter(|(function, _)| {
        // check if relevant channel_control function exists, and remove it from the list
        if channel_filter_regex.is_match(function) {
            let channel_control_function =
                channel_filter_regex.replace(function, "FMOD_ChannelControl_$2");
            !c_functions.contains_key(channel_control_function.as_ref())
        } else {
            true
        }
    }) {
        if category != current_category {
            current_category = category;
            let category = categories.get_index(category).unwrap();
            writeln!(coverage_md, "## {}", category)?;
            if print {
                println!("{}", category.bright_yellow());
            }
        }
        if exists {
            writeln!(coverage_md, "- [x] {name}")?;
            if print {
                println!("{} ({})", name.bright_white(), "🗸".green());
            }
        } else {
            writeln!(coverage_md, "- [ ] {name}")?;
            if print {
                println!("{} ({})", name.bright_white(), "🗴".red())
            }
        }
    }

    coverage_md.flush()?;

    Ok(())
}
