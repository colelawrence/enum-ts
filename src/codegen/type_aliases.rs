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
        // "export type Ok<O, E>"
        src.ln_push("");
        if *export {
            src.push("export ");
        }
        src.push("type ");
        src.push(&t_name);
        src.push(&braced_gen);
        // " = O;"
        src.push(" = ");
        src.push(&contents);
        src.push(";");
    }
}
