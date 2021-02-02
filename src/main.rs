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

export type Stoplight = Enum<{
    Green: 0;
    Yellow: 0;
    Red: 0;
}>;
            "###,
        )), @r###"
        namespace Result {
            export function Ok<O, E>(contents: O): Result<O, E> {
                return ["Ok", contents];
            }
            export function Err<O, E>(contents: E): Result<O, E> {
                return ["Err", contents];
            }
            export function isOk<O, E>(item: Result<O, E>): item is ["Ok", O] {
                return item != null && item[0] === "Ok";
            }
            export function isErr<O, E>(item: Result<O, E>): item is ["Err", E] {
                return item != null && item[0] === "Err";
            }
            export function apply<O, E, R>(fns: {
                Ok(content: O): R;
                Err(content: E): R;
            }): (value: Result<O, E>) => R {
                return function matchResultApply([name, contents]) {
                    // @ts-ignore
                    return fns[name](contents);
                };
            }
            export function match<O, E, R>(
                value: Result<O, E>,
                fns: {
                    Ok(content: O): R;
                    Err(content: E): R;
                }
            ): R {
                return apply(fns)(value);
            }
        }
        export namespace Stoplight {
            export function Green(contents: 0): Stoplight {
                return ["Green", contents];
            }
            export function Yellow(contents: 0): Stoplight {
                return ["Yellow", contents];
            }
            export function Red(contents: 0): Stoplight {
                return ["Red", contents];
            }
            export function isGreen(item: Stoplight): item is ["Green", 0] {
                return item != null && item[0] === "Green";
            }
            export function isYellow(item: Stoplight): item is ["Yellow", 0] {
                return item != null && item[0] === "Yellow";
            }
            export function isRed(item: Stoplight): item is ["Red", 0] {
                return item != null && item[0] === "Red";
            }
            export function apply<R>(fns: {
                Green(content: 0): R;
                Yellow(content: 0): R;
                Red(content: 0): R;
            }): (value: Stoplight) => R {
                return function matchStoplightApply([name, contents]) {
                    // @ts-ignore
                    return fns[name](contents);
                };
            }
            export function match<R>(
                value: Stoplight,
                fns: {
                    Green(content: 0): R;
                    Yellow(content: 0): R;
                    Red(content: 0): R;
                }
            ): R {
                return apply(fns)(value);
            }
        }
        "###)
    }
}
