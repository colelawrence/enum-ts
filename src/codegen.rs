use crate::prelude::*;

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
            // "return ["Ok", contents];"
            create_enum_src.push_end("\n}");
            create_enum_src.ln_push_1("return [\"");
            create_enum_src.push(&t_name);
            create_enum_src.push("\", contents];");

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
            apply_src.push("Apply([name, contents]) {");
            apply_src.ln_push_2("// @ts-ignore");
            apply_src.ln_push_2("return fns[name](contents);");
            apply_src.ln_push_1("};");
            ns_src.push_source_1(apply_src);
            ns_src.push_source_1(match_src);
        }

        if !code.is_empty() {
            code += "\n"
        }
        code.extend(ns_src.finish().drain(..));
    }

    code
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
