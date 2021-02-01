use super::*;

pub(super) fn generate(
    TSEnum {
        generics,
        name,
        variants,
        ..
    }: &TSEnum,
    src: &mut Source,
) {
    // ex "" or "<Ok, Err>"
    let braced_gen = braced_generic(&generics, None);
    // ex "<R>" or "<Ok, Err, R>"
    let braced_gen_r = braced_generic(&generics, Some('R'));
    let mut match_src = src.new_with_same_settings();
    // "export function match<Ok, Err, R>("
    match_src.ln_push("export function match");
    match_src.push(&braced_gen_r);
    match_src.push("(");
    // "value: Result<Ok, Err>,"
    match_src.ln_push_1("value: ");
    match_src.push(&name);
    match_src.push(&braced_gen);
    match_src.push(",");
    match_src.ln_push_1("fns: {");

    let mut apply_src = src.new_with_same_settings();
    // "export function apply<Ok, Err, R>(fns: {"
    apply_src.ln_push("export function apply");
    apply_src.push(&braced_gen_r);
    apply_src.push("(fns: {");
    //
    for (t_name, contents) in variants.iter() {
        // "Ok(content: Ok): R;"
        let mut variant_fn_src = src.new_with_same_settings();
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
    apply_src.push(&braced_gen);
    apply_src.push(") => R {");
    // "return function matchStoplightApply(value) {"
    apply_src.ln_push_1("return function match");
    apply_src.push(&name);
    apply_src.push("Apply([name, contents]) {");
    apply_src.ln_push_2("// @ts-ignore");
    apply_src.ln_push_2("return fns[name](contents);");
    apply_src.ln_push_1("};");
    apply_src.ln_push("}");

    src.push_source(apply_src);
    src.push_source(match_src);
}
