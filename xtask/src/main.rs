use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::owo_colors::OwoColorize;
use indexmap::IndexMap;
use itertools::Itertools;

#[derive(Parser)]
enum Args {
    Coverage {
        core_include_dir: Option<PathBuf>,
        studio_include_dir: Option<PathBuf>,
    },
}

const WRAPPER_H_PATH: &str = "fmod-sys/src/wrapper.h";
const FMOD_OXIDE_DIR: &str = "fmod-oxide/src/";

fn coverage(core_include_dir: PathBuf, studio_include_dir: PathBuf) -> color_eyre::Result<()> {
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
    let mut c_functions: IndexMap<String, bool> = clang::sonar::find_functions(entities)
        .map(|f| (f.name, false))
        .collect();

    for entry in walkdir::WalkDir::new(FMOD_OXIDE_DIR)
        .into_iter()
        .filter_ok(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let entry = std::fs::read_to_string(entry?.path())?;
        let file = syn::parse_file(&entry)?;
        for item in file.items {
            process_item(item, &mut c_functions)
        }
    }

    let mut coverage_md = std::fs::File::create("COVERAGE.md")?;

    let channel_filter_regex = regex::Regex::new("FMOD_(Channel|ChannelGroup)_(.*)$")?;
    for (name, &exists) in c_functions.iter().filter(|(function, _)| {
        // check if relevant channel_control function exists, and remove it from the list
        if channel_filter_regex.is_match(function) {
            let channel_control_function =
                channel_filter_regex.replace(function, "FMOD_ChannelControl_$2");
            !c_functions.contains_key(channel_control_function.as_ref())
        } else {
            true
        }
    }) {
        if exists {
            writeln!(coverage_md, "- [x] {name}")?;
            println!("{} ({})", name.bright_white(), "🗸".green());
        } else {
            writeln!(coverage_md, "- [ ] {name}")?;
            println!("{} ({})", name.bright_white(), "🗴".red())
        }
    }

    coverage_md.flush()?;

    Ok(())
}

fn process_item(item: syn::Item, c_functions: &mut IndexMap<String, bool>) {
    match item {
        syn::Item::Fn(item) => process_block(*item.block, c_functions),
        syn::Item::Impl(item) => {
            for item in item.items.into_iter().filter_map(impl_item_into_fn) {
                process_block(item.block, c_functions)
            }
        }
        // we should NEVER run across a foreign item in the fmod-oxide crate
        syn::Item::Mod(item) => {
            if let Some((_, items)) = item.content {
                for item in items {
                    process_item(item, c_functions)
                }
            }
        }
        _ => {}
    }
}

fn process_block(block: syn::Block, c_functions: &mut IndexMap<String, bool>) {
    for stmt in block.stmts {
        match stmt {
            syn::Stmt::Item(item) => process_item(item, c_functions),
            syn::Stmt::Expr(expr, _) => process_expr(expr, c_functions),
            syn::Stmt::Local(local) => {
                if let Some(init) = local.init {
                    process_expr(*init.expr, c_functions);
                    if let Some((_, expr)) = init.diverge {
                        process_expr(*expr, c_functions)
                    }
                }
            }
            // not really sure what to do with these
            syn::Stmt::Macro(_) => {}
        }
    }
}

fn process_expr(expr: syn::Expr, c_functions: &mut IndexMap<String, bool>) {
    match expr {
        syn::Expr::Array(expr) => {
            for elem in expr.elems {
                process_expr(elem, c_functions)
            }
        }
        syn::Expr::Assign(expr) => {
            process_expr(*expr.left, c_functions);
            process_expr(*expr.right, c_functions)
        }
        syn::Expr::Async(expr) => process_block(expr.block, c_functions),
        syn::Expr::Await(expr) => process_expr(*expr.base, c_functions),
        syn::Expr::Binary(expr) => {
            process_expr(*expr.left, c_functions);
            process_expr(*expr.right, c_functions)
        }
        syn::Expr::Block(expr) => process_block(expr.block, c_functions),
        syn::Expr::Break(expr) => {
            if let Some(expr) = expr.expr {
                process_expr(*expr, c_functions)
            }
        }
        syn::Expr::Call(expr) => {
            process_expr(*expr.func, c_functions);
            for arg in expr.args {
                process_expr(arg, c_functions)
            }
        }
        syn::Expr::Cast(expr) => process_expr(*expr.expr, c_functions),
        syn::Expr::Closure(expr) => process_expr(*expr.body, c_functions),
        // don't need to process const blocks as none of the ffi functions are const
        // continue has no expr to process
        syn::Expr::Field(expr) => process_expr(*expr.base, c_functions),
        syn::Expr::ForLoop(expr) => {
            process_expr(*expr.expr, c_functions);
            process_block(expr.body, c_functions);
        }
        // no clue what this is
        syn::Expr::Group(expr) => process_expr(*expr.expr, c_functions),
        syn::Expr::If(expr) => {
            process_expr(*expr.cond, c_functions);
            process_block(expr.then_branch, c_functions);
            if let Some((_, expr)) = expr.else_branch {
                process_expr(*expr, c_functions);
            }
        }
        syn::Expr::Index(expr) => {
            process_expr(*expr.expr, c_functions);
            process_expr(*expr.index, c_functions);
        }
        // infer has no expr to process
        syn::Expr::Let(expr) => process_expr(*expr.expr, c_functions),
        // literal has no expr to process
        syn::Expr::Loop(expr) => process_block(expr.body, c_functions),
        // cant really do anything with macros
        syn::Expr::Match(expr) => {
            process_expr(*expr.expr, c_functions);
            for arm in expr.arms {
                if let Some((_, expr)) = arm.guard {
                    process_expr(*expr, c_functions)
                }
                process_expr(*arm.body, c_functions)
            }
        }
        syn::Expr::MethodCall(expr) => {
            // none of the c functions are methods, so we don't care about the method name
            process_expr(*expr.receiver, c_functions);
            for arg in expr.args {
                process_expr(arg, c_functions)
            }
        }
        syn::Expr::Paren(expr) => process_expr(*expr.expr, c_functions),
        // TODO figure out what to do with path
        syn::Expr::Path(path) => {
            for segment in path.path.segments {
                let ident = segment.ident.to_string();
                if let Some(exists) = c_functions.get_mut(&ident) {
                    *exists = true;
                }
            }
        }
        syn::Expr::Range(expr) => {
            if let Some(start) = expr.start {
                process_expr(*start, c_functions)
            }
            if let Some(end) = expr.end {
                process_expr(*end, c_functions)
            }
        }
        syn::Expr::Reference(expr) => process_expr(*expr.expr, c_functions),
        syn::Expr::Repeat(expr) => {
            process_expr(*expr.expr, c_functions);
            process_expr(*expr.len, c_functions)
        }
        syn::Expr::Return(expr) => {
            if let Some(expr) = expr.expr {
                process_expr(*expr, c_functions)
            }
        }
        syn::Expr::Struct(expr) => {
            for field in expr.fields {
                process_expr(field.expr, c_functions)
            }
            if let Some(expr) = expr.rest {
                process_expr(*expr, c_functions)
            }
        }
        syn::Expr::Try(expr) => process_expr(*expr.expr, c_functions),
        syn::Expr::TryBlock(expr) => process_block(expr.block, c_functions),
        syn::Expr::Tuple(expr) => {
            for elem in expr.elems {
                process_expr(elem, c_functions)
            }
        }
        syn::Expr::Unary(expr) => process_expr(*expr.expr, c_functions),
        syn::Expr::Unsafe(expr) => process_block(expr.block, c_functions),
        // can we process verbatim..?
        syn::Expr::While(expr) => {
            process_expr(*expr.cond, c_functions);
            process_block(expr.body, c_functions)
        }
        syn::Expr::Yield(expr) => {
            if let Some(expr) = expr.expr {
                process_expr(*expr, c_functions)
            }
        }
        _ => {}
    }
}

fn impl_item_into_fn(item: syn::ImplItem) -> Option<syn::ImplItemFn> {
    if let syn::ImplItem::Fn(item) = item {
        Some(item)
    } else {
        None
    }
}

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();
    match args {
        Args::Coverage {
            core_include_dir,
            studio_include_dir,
        } => {
            let core_include_dir =
                core_include_dir.unwrap_or_else(|| PathBuf::from("fmod/api/core/inc"));
            let studio_include_dir =
                studio_include_dir.unwrap_or_else(|| PathBuf::from("fmod/api/studio/inc"));
            if let Err(e) = coverage(core_include_dir, studio_include_dir) {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}
