use once_cell::sync::Lazy;
use regex::Regex;
use std::hash::Hash;

#[derive(Debug, Hash)]
pub struct TSEnum {
    // type name
    pub name: String,
    // list of generics sans < > if there are generics
    pub generics: Option<String>,
    // t & c pairs
    pub variants: Vec<(String, String)>,
    pub export: bool,
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

#[derive(Debug, Hash)]
pub struct Parsed {
    pub enums: Vec<TSEnum>,
    pub indent: String,
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
                    line.is_empty() || unindented != line,
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

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

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
