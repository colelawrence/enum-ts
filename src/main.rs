use ignore::WalkBuilder;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{env, io};

mod codegen;
mod parser;
mod string_utils;
mod write;

pub use codegen::*;
pub use parser::*;
pub use write::*;

pub(crate) use string_utils::*;

pub(crate) mod prelude {
    pub use crate::{Parsed, TSEnum};
}

fn main() {
    let mode = match args::Mode::from_args(
        env::current_dir().expect("Something went wrong looking up current directory"),
        env::args().collect(),
    ) {
        Ok(mode) => mode,
        Err(err) => exit_err(err),
    };

    match mode {
        args::Mode::Pipe(mode) => pipe_mode(mode),
        args::Mode::Write(write_options) => write_mode(write_options),
    }
}

fn pipe_mode(mode: args::PipeMode) {
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        match stdin.read_line(&mut input) {
            Ok(0) => {
                match mode {
                    args::PipeMode::ShowGenerated => {
                        let parsed = parse(&input);
                        println!("{}", generate(parsed));
                    }
                    args::PipeMode::ShowReplaceRangeVSCode => {
                        if let Some((start, end, to_write)) = make_edit(&input, true) {
                            eprintln!(
                                "update-range: L{}:{}-L{}:{}",
                                start.line, start.col, end.line, end.col
                            );
                            println!("{}", to_write);
                        } else {
                            eprintln!("no-update");
                        }
                    }
                    args::PipeMode::ShowFullFile => {
                        if let Some(to_write) = rewrite(&input, true) {
                            eprintln!("Updated");
                            println!("{}", to_write);
                        } else {
                            eprintln!("No change");
                            println!("{}", input);
                        }
                    }
                }
                return;
            }
            Ok(_number_of_bytes_read) => {
                // collected into input
            }
            Err(error) => eprintln!("error: {}", error),
        }
    }
}

// Matches TypeScript files
static RE_TYPESCRIPT_FILE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.[Tt][Ss][Xx]?$").unwrap());

fn write_mode(mut options: args::WriteOptions) {
    let mut paths_it = options.paths.drain(..);

    let mut walk_builder = if let Some(path) = paths_it.next() {
        let mut walk_builder = WalkBuilder::new(options.base_dir.join(&path));
        for path in paths_it {
            walk_builder.add(options.base_dir.join(&path));
        }
        walk_builder
    } else {
        WalkBuilder::new(&options.base_dir)
    };

    for ignore in options.ignore_files {
        let ignore_file = options.base_dir.join(&ignore);
        if let Some(err) = walk_builder.add_ignore(ignore_file) {
            exit_err(err);
        }
    }

    let write = !options.dry_run;
    let force = options.force_updates;
    walk_builder.build_parallel().run(|| {
        Box::new(
            |entry: std::result::Result<ignore::DirEntry, ignore::Error>| match &entry {
                Ok(dir) => {
                    if let Some(file_type) = dir.file_type() {
                        if file_type.is_file()
                            && RE_TYPESCRIPT_FILE.is_match(&dir.file_name().to_string_lossy())
                        {
                            rewrite_file(dir.path(), write, force);
                        }
                    }
                    ignore::WalkState::Continue
                }
                Err(err) => {
                    eprintln!("Error encountered (skipping): {}", err);
                    ignore::WalkState::Skip
                }
            },
        )
    })
}

fn exit_err<E: std::fmt::Display>(err: E) -> ! {
    eprintln!("Failed to execute enum-ts\n{}", err);
    std::process::exit(1);
}

mod args {
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct WriteOptions {
        pub dry_run: bool,
        pub force_updates: bool,
        pub base_dir: PathBuf,
        pub paths: Vec<String>,
        pub ignore_files: Vec<String>,
        // eventually support globs and stuff https://docs.rs/ignore/0.4.17/ignore/overrides/index.html
        // pub ignore: Vec<String>,
    }

    #[derive(Debug)]
    pub enum PipeMode {
        ShowGenerated,
        ShowReplaceRangeVSCode,
        ShowFullFile,
    }

    pub enum Mode {
        Pipe(PipeMode),
        Write(WriteOptions),
    }

    impl Mode {
        pub fn from_args(cwd: PathBuf, args: Vec<String>) -> Result<Self, String> {
            let mut args_iterator = args.into_iter().peekable();
            let _executable = args_iterator.next();
            match args_iterator.peek() {
                Some(arg) if arg == "--edit-l1c0" => {
                    // one arg for edit mode
                    return Ok(Mode::Pipe(PipeMode::ShowReplaceRangeVSCode));
                }
                Some(arg) if arg == "--full" => {
                    // one arg for edit mode
                    return Ok(Mode::Pipe(PipeMode::ShowFullFile));
                }
                None => {
                    // no args will always be pipe mode
                    return Ok(Mode::Pipe(PipeMode::ShowGenerated));
                }
                _ => {}
            }

            let mut write_options = WriteOptions {
                dry_run: true,
                force_updates: false,
                base_dir: cwd,
                paths: Vec::new(),
                // ignore: Vec::new(),
                ignore_files: Vec::new(),
            };

            let mut errors = Vec::new();

            while let Some(next_option) = args_iterator.next() {
                match next_option.as_str() {
                    "-w" | "--write" => {
                        write_options.dry_run = false;
                    }
                    "-f" | "--force" => {
                        write_options.force_updates = true;
                    }
                    "--ignore-file" => {
                        if let Some(ignore_file) = args_iterator.next() {
                            write_options.ignore_files.push(ignore_file);
                        } else {
                            errors.push(format!("File must be specified following the `{}` argument like `{} .ignore-file`", &next_option, &next_option));
                        }
                    }
                    other => {
                        if other.starts_with('-') {
                            errors.push(format!("Unknown option: {:?}", other));
                        } else {
                            write_options.paths.push(next_option);
                        }
                    }
                }
            }

            if !errors.is_empty() {
                super::exit_err(format!("Errors:\n - {}", errors.join("\n - ")));
            }

            Ok(Mode::Write(write_options))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_display_snapshot;

    #[test]
    fn parse_result_with_generics_generate() {
        assert_display_snapshot!(generate(parse(
            r###"
type Result<O, E> = Enum<{
    Ok: O;
    Err: E;
}>;

export type Nested = Enum<{
    Leaf: 0;
    Branch: {
        left: Nested,
        right: Nested,
    }
}>;

export type Stoplight = Enum<{
    Green: 0;
    Yellow: 0;
    Red: 0;
}>;
            "###,
        )), @r###"
        type Ok<O, E> = O;
        type Err<O, E> = E;
        function Ok<O, E>(contents: Ok<O, E>): { Ok: Ok<O, E> } {
            return { Ok: contents };
        }
        function Err<O, E>(contents: Err<O, E>): { Err: Err<O, E> } {
            return { Err: contents };
        }
        function isOk<O, E>(item: Result<O, E>): item is { Ok: Ok<O, E> } {
            return item != null && "Ok" in item;
        }
        function isErr<O, E>(item: Result<O, E>): item is { Err: Err<O, E> } {
            return item != null && "Err" in item;
        }
        namespace Result {
            const unexpected = "Unexpected Enum variant for Result<O, E>";
            export function apply<O, E, R>(fns: {
                Ok(content: Ok<O, E>): R;
                Err(content: Err<O, E>): R;
            }): (value: Result<O, E>) => R {
                return function matchResultApply(item) {
                    return "Ok" in item
                        ? fns.Ok(item.Ok)
                        : "Err" in item
                        ? fns.Err(item.Err)
                        : (console.assert(false, unexpected, item) as never);
                };
            }
            export function match<O, E, R>(
                value: Result<O, E>,
                fns: {
                    Ok(content: Ok<O, E>): R;
                    Err(content: Err<O, E>): R;
                }
            ): R {
                return apply(fns)(value);
            }
        }

        export type Leaf = 0;
        export type Branch = {
            left: Nested,
            right: Nested,
        };
        export function Leaf(contents: Leaf): { Leaf: Leaf } {
            return { Leaf: contents };
        }
        export function Branch(contents: Branch): { Branch: Branch } {
            return { Branch: contents };
        }
        export function isLeaf(item: Nested): item is { Leaf: Leaf } {
            return item != null && "Leaf" in item;
        }
        export function isBranch(item: Nested): item is { Branch: Branch } {
            return item != null && "Branch" in item;
        }
        export namespace Nested {
            const unexpected = "Unexpected Enum variant for Nested";
            export function apply<R>(fns: {
                Leaf(content: Leaf): R;
                Branch(content: Branch): R;
            }): (value: Nested) => R {
                return function matchNestedApply(item) {
                    return "Leaf" in item
                        ? fns.Leaf(item.Leaf)
                        : "Branch" in item
                        ? fns.Branch(item.Branch)
                        : (console.assert(false, unexpected, item) as never);
                };
            }
            export function match<R>(
                value: Nested,
                fns: {
                    Leaf(content: Leaf): R;
                    Branch(content: Branch): R;
                }
            ): R {
                return apply(fns)(value);
            }
        }

        export type Green = 0;
        export type Yellow = 0;
        export type Red = 0;
        export function Green(contents: Green): { Green: Green } {
            return { Green: contents };
        }
        export function Yellow(contents: Yellow): { Yellow: Yellow } {
            return { Yellow: contents };
        }
        export function Red(contents: Red): { Red: Red } {
            return { Red: contents };
        }
        export function isGreen(item: Stoplight): item is { Green: Green } {
            return item != null && "Green" in item;
        }
        export function isYellow(item: Stoplight): item is { Yellow: Yellow } {
            return item != null && "Yellow" in item;
        }
        export function isRed(item: Stoplight): item is { Red: Red } {
            return item != null && "Red" in item;
        }
        export namespace Stoplight {
            const unexpected = "Unexpected Enum variant for Stoplight";
            export function apply<R>(fns: {
                Green(content: Green): R;
                Yellow(content: Yellow): R;
                Red(content: Red): R;
            }): (value: Stoplight) => R {
                return function matchStoplightApply(item) {
                    return "Green" in item
                        ? fns.Green(item.Green)
                        : "Yellow" in item
                        ? fns.Yellow(item.Yellow)
                        : "Red" in item
                        ? fns.Red(item.Red)
                        : (console.assert(false, unexpected, item) as never);
                };
            }
            export function match<R>(
                value: Stoplight,
                fns: {
                    Green(content: Green): R;
                    Yellow(content: Yellow): R;
                    Red(content: Red): R;
                }
            ): R {
                return apply(fns)(value);
            }
        }
        "###)
    }
}
