use super::*;

pub(super) fn generate(
    TSEnum {
        export,
        generics,
        name,
        variants,
        ..
    }: &TSEnum,
    src: &mut Source,
) {
    let braced_gen = braced_generic(&generics, None);
    for (t_name, _) in variants.iter() {
        // "export function isOk<O, E>("
        src.ln_push("");
        if *export {
            src.push("export ");
        }
        src.push("function is");
        src.push(&t_name);
        src.push(&braced_gen);
        src.push("(");
        // "item: Result<O, E>"
        src.push("item: ");
        src.push(&name);
        src.push(&braced_gen);
        // "): item is { Ok: O } {"
        src.push("): item is { ");
        src.push(&t_name);
        src.push(": ");
        src.push(&t_name);
        src.push(&braced_gen);
        src.push(" } {");
        // "return item != null && "Ok" in item;"
        src.ln_push_1("return item != null && \"");
        src.push(&t_name);
        src.push("\" in item;");
        src.ln_push("}");
    }
}
