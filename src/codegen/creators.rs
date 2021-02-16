use super::*;

pub(super) fn generate(
    TSEnum {
        export,
        generics,
        variants,
        ..
    }: &TSEnum,
    src: &mut Source,
) {
    // ex "" or "<Ok, Err>"
    let braced_gen = braced_generic(&generics, None);
    for (t_name, contents) in variants.iter() {
        // "export function Ok<O, E>("
        src.ln_push("");
        if *export {
            src.push("export ");
        }
        src.push("function ");
        src.push(&t_name);
        src.push(&braced_gen);
        src.push("(");
        if contents != "null" {
            // "contents: Ok<O, E>"
            src.push("contents: ");
            // note: should be defined by type_aliases
            src.push(&t_name);
            src.push(&braced_gen);
        }
        // "): { Ok: Ok<O, E> } {"
        src.push("): { ");
        src.push(&t_name);
        src.push(": ");
        src.push(&t_name);
        src.push(&braced_gen);
        src.push(" } {");
        // "return { Ok: contents };"
        src.ln_push_1("return { ");
        src.push(&t_name);
        if contents != "null" {
            src.push(": contents };");
        } else {
            src.push(": null };");
        }
        src.ln_push("}");
    }
}
