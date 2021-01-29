use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub struct TSEnum {
    // type name
    name: String,
    // list of generics sans < > if there are generics
    generics: Option<String>,
    // t & c pairs
    variants: Vec<(String, String)>,
    export: bool,
}

// Only matches enums which are on the first level
static RE_ENUM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\n(?P<export>export\s+)?type[\s]+(?P<name>\w+)(?:<(?P<generics>[\w\s,]+)>)?\s*=\s*Enum<\{(?P<variants>[\s\S]+?)\n\}>").unwrap()
});
static RE_VARIANTS_INDENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\n([\t ]+)").unwrap());
// after normalized with indent
static RE_VARIANT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\n(?P<name>\w+):\s*(?P<contents>[^\n;,]+(?:\n[ \t]+[\s\S]+?\n[\]\}>]+)?)").unwrap()
});

#[derive(Debug)]
pub struct Parsed {
    enums: Vec<TSEnum>,
    indent: String,
}

pub fn parse(source: &str) -> Parsed {
    let mut enums = Vec::new();
    let mut indent = String::new();
    for cap in RE_ENUM.captures_iter(&source) {
        let variants: &str = &cap["variants"];
        let indent_match: &str = &RE_VARIANTS_INDENT
            .captures(&variants)
            .expect("at least one variant + indented")[1];
        let unindented_variants: String = variants
            .lines()
            .into_iter()
            .map(|line| {
                let unindented = line.replace(indent_match, "\n");
                assert!(
                    line.len() == 0 || unindented != line,
                    "line indentation is irregular:>>>{}<<<",
                    line
                );
                unindented
            })
            .collect();
        indent = indent_match.to_string();

        enums.push(TSEnum {
            name: cap["name"].to_string(),
            generics: cap.name("generics").map(|val| val.as_str().to_string()),
            export: cap.name("export").is_some(),
            variants: RE_VARIANT
                .captures_iter(&unindented_variants)
                .map(|cap| (cap["name"].to_string(), cap["contents"].to_string()))
                .collect(),
        });
    }

    Parsed { indent, enums }
}

#[derive(Clone)]
struct Source {
    indent: String,
    code: String,
    end: String,
}

impl Source {
    fn new(indent: String) -> Self {
        Source {
            indent,
            code: String::new(),
            end: String::new(),
        }
    }
    fn push(&mut self, s: &str) {
        self.code.push_str(s);
    }
    fn push_end(&mut self, s: &str) {
        self.end.push_str(s);
    }
    fn ln_push(&mut self, s: &str) {
        self.code.push_str("\n");
        self.code.push_str(s);
    }
    fn ln_push_1(&mut self, s: &str) {
        self.code.push_str("\n");
        self.code.push_str(&self.indent);
        self.code.push_str(s);
    }
    fn ln_push_2(&mut self, s: &str) {
        self.code.push_str("\n");
        self.code.push_str(&self.indent);
        self.code.push_str(&self.indent);
        self.code.push_str(s);
    }
    fn push_source_1(&mut self, other: Self) {
        let indent_1 = "\n".to_owned() + &self.indent;
        self.code
            .extend(other.finish().replace("\n", &indent_1).drain(..));
    }
    fn push_source_2(&mut self, other: Self) {
        let indent_2 = "\n".to_owned() + &self.indent + &self.indent;
        self.code
            .extend(other.finish().replace("\n", &indent_2).drain(..));
    }
    fn finish(mut self) -> String {
        self.code.extend(self.end.drain(..));
        self.code
    }
}

pub fn generate(Parsed { enums, indent }: Parsed) -> String {
    let mut code = String::new();
    for TSEnum {
        generics,
        name,
        variants,
        export,
    } in enums
    {
        let mut ns_src = Source::new(indent.clone());
        ns_src.ln_push("");
        if export {
            ns_src.push("export ");
        }
        ns_src.push("namespace ");
        ns_src.push(&name);
        ns_src.push(" {");
        ns_src.push_end("\n}");
        // ex "" or "<Ok, Err>"
        let gen = generics.as_ref().map_or_else(
            || String::new(),
            |gen| {
                let mut s = String::from("<");
                s.push_str(&gen);
                s.push_str(">");
                s
            },
        );
        for (t_name, contents) in variants.iter() {
            let mut create_enum_src = Source::new(indent.clone());
            // "export function Ok<O, E>"
            create_enum_src.ln_push("export function ");
            create_enum_src.push(&t_name);
            create_enum_src.push(&gen);
            // "(contents: Ok): Result<O, E> {"
            create_enum_src.push("(contents: ");
            create_enum_src.push(&contents);
            create_enum_src.push("): ");
            create_enum_src.push(&name);
            create_enum_src.push(&gen);
            create_enum_src.push(" {");
            // "return { t: "Ok", c: contents };"
            create_enum_src.push_end("\n}");
            create_enum_src.ln_push_1("return { t: \"");
            create_enum_src.push(&t_name);
            create_enum_src.push("\", c: contents };");

            ns_src.push_source_1(create_enum_src);
        }

        {
            let mut match_src = Source::new(indent.clone());
            // "export function match<Ok, Err, R>("
            match_src.ln_push("export function match<");
            if let Some(gen_args) = &generics {
                match_src.push(&gen_args);
                match_src.push(", ");
            }
            match_src.push("R>(");
            // "value: Result<Ok, Err>,"
            match_src.ln_push_1("value: ");
            match_src.push(&name);
            match_src.push(&gen);
            match_src.push(",");
            match_src.ln_push_1("fns: {");
            
            let mut apply_src = Source::new(indent.clone());
            // "export function apply<Ok, Err, R>(fns: {"
            apply_src.ln_push("export function apply<");
            if let Some(gen_args) = &generics {
                apply_src.push(&gen_args);
                apply_src.push(", ");
            }
            apply_src.push("R>(fns: {");
            //
            for (t_name, contents) in variants.iter() {
                // "Ok(content: Ok): R;"
                let mut variant_fn_src = Source::new(indent.clone());
                variant_fn_src.ln_push(&t_name);
                variant_fn_src.push("(content: ");
                variant_fn_src.push(&contents);
                variant_fn_src.push("): R;");

                apply_src.push_source_1(variant_fn_src.clone());
                match_src.push_source_2(variant_fn_src);
            }
            match_src.ln_push_1("}");
            match_src.ln_push("): R {");
            match_src.ln_push_1("return apply(fns)(value);");
            match_src.ln_push("}");

            apply_src.push("\n}): (value: ");
            apply_src.push(&name);
            apply_src.push(&gen);
            apply_src.push(") => R {");
            apply_src.push_end("\n}");
            // "return function matchStoplightApply(value) {"
            apply_src.ln_push_1("return function match");
            apply_src.push(&name);
            apply_src.push("Apply(value) {");
            apply_src.ln_push_2("// @ts-ignore");
            apply_src.ln_push_2("return fns[value.t](value.c);");
            apply_src.ln_push_1("};");
            ns_src.push_source_1(apply_src);
            ns_src.push_source_1(match_src);
        }

        code.extend(ns_src.finish().drain(..));
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use insta::assert_display_snapshot;

    #[test]
    fn parse_result_with_generics_generate() {
        assert_display_snapshot!(generate(parse(
            r###"
// enum: factory, match
type Result<O, E> = Enum<{
    Ok: O;
    Err: E;
}>;

// enum: factory, match
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

    #[test]
    fn parse_result_with_generics() {
        assert_debug_snapshot!(parse(
            r###"
// enum: factory, match
type Result<Ok, Err> = Enum<{
    Ok: Ok;
    Err: Err;
}>;

// enum: factory, match
type Stoplight = Enum<{
    Green: 0;
    Yellow: 0;
    Red: 0;
}>;
            "###,
        ), @r###"
        Parsed {
            enums: [
                TSEnum {
                    name: "Result",
                    generics: Some(
                        "Ok, Err",
                    ),
                    variants: [
                        (
                            "Ok",
                            "Ok",
                        ),
                        (
                            "Err",
                            "Err",
                        ),
                    ],
                    export: false,
                },
                TSEnum {
                    name: "Stoplight",
                    generics: None,
                    variants: [
                        (
                            "Green",
                            "0",
                        ),
                        (
                            "Yellow",
                            "0",
                        ),
                        (
                            "Red",
                            "0",
                        ),
                    ],
                    export: false,
                },
            ],
            indent: "    ",
        }
        "###)
    }
}
