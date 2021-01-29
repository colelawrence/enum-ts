use crate::*;

use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::hash::{Hash, Hasher};
use std::{collections::hash_map::DefaultHasher, path::Path};

// Only matches enums which are on the first level
static PREFIX_PRE_HASH: &str = "\n//#region enum-ts generated <";
static PREFIX_POST_HASH: &str = ">\n";
static SUFFIX: &str = "\n//#endregion";

static RE_ENUM_TS_REGION: Lazy<Regex> = Lazy::new(|| {
    let mut source = regex::escape(PREFIX_PRE_HASH);
    source.push_str(r"(?P<hash>\w*)"); // maybe some hash
    source.extend(regex::escape(PREFIX_POST_HASH).drain(..));
    source.push_str(r"[\s\S]+?"); // everything non-greedy
    source.extend(regex::escape(SUFFIX).drain(..));

    Regex::new(&source).unwrap()
});

pub fn rewrite(contents: &str) -> Option<String> {
    let parsed = parse(contents);
    if parsed.enums.is_empty() {
        // no enums to generate
        return None;
    }
    let mut hasher = DefaultHasher::new();
    parsed.hash(&mut hasher);
    let hash_str = format!("{:x}", hasher.finish());
    let prefix: String = String::from(PREFIX_PRE_HASH) + &hash_str + PREFIX_POST_HASH;
    if contents.contains(&prefix) {
        return None;
    } else {
        let mut to_write = prefix;
        to_write.extend(generate(parsed).drain(..));
        to_write.push_str(&SUFFIX);

        let mut to_write_applied = Some(to_write);
        let mut applied = RE_ENUM_TS_REGION
            .replace_all(&contents, |_: &regex::Captures| {
                to_write_applied.take().unwrap_or_default()
            })
            .to_string();

        if let Some(mut to_write_still) = to_write_applied {
            // it wasn't replaced in the document, so put it at the end
            applied.extend(to_write_still.drain(..));
        }

        Some(applied)
    }
}

pub fn rewrite_file<P: AsRef<Path>>(path: P, write: bool) -> bool {
    let path_ref = path.as_ref();
    let file_contents =
        fs::read_to_string(&path_ref).expect("Something went wrong reading the file");
    if let Some(substitution) = rewrite(&file_contents) {
        if write {
            fs::write(&path, substitution).expect("Something went wrong writing the file");
            println!("Wrote: {}", path_ref.to_string_lossy());
        } else {
            println!(
                "Would write: {}\n<<enum-ts-dry-run>>\n{}\n<</enum-ts-dry-run>>",
                &path_ref.to_string_lossy(),
                &substitution,
            );
        }
        true
    } else {
        false
    }
}
