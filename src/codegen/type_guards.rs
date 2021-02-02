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
    let braced_gen = braced_generic(&generics, None);
    for (t_name, contents) in variants.iter() {
        // "export function isOk<O, E>"
        src.ln_push("export function is");
        src.push(&t_name);
        src.push(&braced_gen);
        // "(item: Result<O, E>): item is ["Ok", O] {"
        src.push("(item: ");
        src.push(&name);
        src.push(&braced_gen);
        src.push("): item is [\"");
        src.push(&t_name);
        src.push("\", ");
        src.push(&contents);
        src.push("] {");
        // "return item != null && item[0] === "Ok";"
        src.ln_push_1("return item != null && item[0] === \"");
        src.push(&t_name);
        src.push("\";");
        src.ln_push("}");
    }
}
