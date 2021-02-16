use crate::prelude::*;

mod apply_match;
mod creators;
mod type_aliases;
mod type_guards;

// if the enum generated type structure ever updates, then increment this
pub const CODE_GEN_VERSION: usize = 6;
pub fn generate(Parsed { enums, indent }: Parsed) -> String {
    let mut code = String::new();
    for ts_enum in enums {
        let TSEnum { name, export, .. } = &ts_enum;

        let mut ns_src = Source::new(indent.clone());
        type_aliases::generate(&ts_enum, &mut ns_src);
        creators::generate(&ts_enum, &mut ns_src);
        type_guards::generate(&ts_enum, &mut ns_src);

        ns_src.ln_push("");
        // "export namespace Result {"
        if *export {
            ns_src.push("export ");
        }
        ns_src.push("namespace ");
        ns_src.push(&name);
        ns_src.push(" {");

        let mut nested_src = ns_src.new_with_same_settings();
        apply_match::generate(&ts_enum, &mut nested_src);
        ns_src.push_source_1(nested_src);
        ns_src.ln_push("}");

        if !code.is_empty() {
            code += "\n"
        }
        code.extend(ns_src.finish().drain(..));
    }

    code.trim().to_string()
}

/// Generates `"<O, E>"` or `""` or `"<O, E, R>"` or "<R>" depending on params
fn braced_generic(generics: &Option<String>, extra_generic_opt: Option<char>) -> String {
    generics.as_ref().map_or_else(
        || {
            extra_generic_opt.map_or_else(String::new, |extra| {
                let mut s = String::from("<");
                s.push(extra);
                s.push_str(">");
                s
            })
        },
        |gen| {
            let mut s = String::from("<");
            s.push_str(&gen);
            if let Some(extra_generic) = extra_generic_opt {
                s.push_str(", ");
                s.push(extra_generic);
            }
            s.push_str(">");
            s
        },
    )
}

#[cfg(test)]
mod tests {
    use super::braced_generic;

    #[test]
    fn test_braced_generic() {
        assert_eq!(braced_generic(&None, None), s(""));
        assert_eq!(braced_generic(&Some(s("A")), None), s("<A>"));
        assert_eq!(braced_generic(&Some(s("A")), Some('R')), s("<A, R>"));
        assert_eq!(braced_generic(&None, Some('R')), s("<R>"));
        assert_eq!(
            braced_generic(&Some(s("A, B, C")), Some('R')),
            s("<A, B, C, R>")
        );
    }

    fn s(string: &str) -> String {
        string.into()
    }
}

#[derive(Clone)]
struct Source {
    indent: String,
    code: String,
}

impl Source {
    fn new(indent: String) -> Self {
        Source {
            indent,
            code: String::new(),
        }
    }
    fn new_with_same_settings(&self) -> Self {
        Source {
            code: String::new(),
            indent: self.indent.clone(),
        }
    }
    fn push(&mut self, s: &str) {
        self.code.push_str(s);
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
    fn push_source(&mut self, other: Self) {
        self.code.extend(other.finish().drain(..));
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
    fn finish(self) -> String {
        self.code
    }
}
