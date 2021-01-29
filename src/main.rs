use ignore::WalkBuilder;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{env, io};

mod codegen;
mod parser;
mod write;

pub use codegen::*;
pub use parser::*;
pub use write::*;

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
        args::Mode::Pipe => pipe_mode(),
        args::Mode::Write(write_options) => write_mode(write_options),
    }
}

fn pipe_mode() {
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        match stdin.read_line(&mut input) {
            Ok(0) => {
                let parsed = parse(&input);
                println!("{}", generate(parsed));
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
    walk_builder.build_parallel().run(|| {
        Box::new(
            |entry: std::result::Result<ignore::DirEntry, ignore::Error>| match &entry {
                Ok(dir) => {
                    if let Some(file_type) = dir.file_type() {
                        if file_type.is_file() {
                            if RE_TYPESCRIPT_FILE.is_match(&dir.file_name().to_string_lossy()) {
                                rewrite_file(dir.path(), write);
                            }
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
        pub base_dir: PathBuf,
        pub paths: Vec<String>,
        pub ignore_files: Vec<String>,
        // eventually support globs and stuff https://docs.rs/ignore/0.4.17/ignore/overrides/index.html
        // pub ignore: Vec<String>,
    }

    pub enum Mode {
        Pipe,
        Write(WriteOptions),
    }

    impl Mode {
        pub fn from_args(cwd: PathBuf, args: Vec<String>) -> Result<Self, String> {
            let mut args_iterator = args.into_iter().peekable();
            let _executable = args_iterator.next();
            if args_iterator.peek().is_none() {
                // no args will always be pipe mode
                return Ok(Mode::Pipe);
            }
            let mut write_options = WriteOptions {
                dry_run: true,
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
                    "--ignore-file" => {
                        if let Some(ignore_file) = args_iterator.next() {
                            write_options.ignore_files.push(ignore_file);
                        } else {
                            errors.push(format!("File must be specified following the `{}` argument like `{} .ignore-file`", &next_option, &next_option));
                        }
                    }
                    other => {
                        if other.starts_with("-") {
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
                return { t: "Ok", c: contents };
            }
            export function Err<O, E>(contents: E): Result<O, E> {
                return { t: "Err", c: contents };
            }
            export function apply<O, E, R>(fns: {
                Ok(content: O): R;
                Err(content: E): R;
            }): (value: Result<O, E>) => R {
                return function matchResultApply(value) {
                    // @ts-ignore
                    return fns[value.t](value.c);
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
                return { t: "Green", c: contents };
            }
            export function Yellow(contents: 0): Stoplight {
                return { t: "Yellow", c: contents };
            }
            export function Red(contents: 0): Stoplight {
                return { t: "Red", c: contents };
            }
            export function apply<R>(fns: {
                Green(content: 0): R;
                Yellow(content: 0): R;
                Red(content: 0): R;
            }): (value: Stoplight) => R {
                return function matchStoplightApply(value) {
                    // @ts-ignore
                    return fns[value.t](value.c);
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
