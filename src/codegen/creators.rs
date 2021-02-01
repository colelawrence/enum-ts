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
    for (t_name, contents) in variants.iter() {
        // "export function Ok<O, E>"
        src.ln_push("export function ");
        src.push(&t_name);
        src.push(&braced_gen);
        // "(contents: Ok): Result<O, E> {"
        src.push("(contents: ");
        src.push(&contents);
        src.push("): ");
        src.push(&name);
        src.push(&braced_gen);
        src.push(" {");
        // "return ["Ok", contents];"
        src.ln_push_1("return [\"");
        src.push(&t_name);
        src.push("\", contents];");
        src.ln_push("}");
    }
}
