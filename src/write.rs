use crate::*;

use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::hash::{Hash, Hasher};
use std::{collections::hash_map::DefaultHasher, path::Path};

// if the enum type structure ever updates, then increment this
const ENUM_STRUCTURE_VERSION: usize = 2;
// Only matches enums which are on the first level
static PREFIX_PRE_HASH: &str = "\n//#region enum-ts generated <";
static PREFIX_POST_HASH: &str = ">\n";
static SUFFIX: &str = "\n//#endregion";

static RE_ENUM_TS_REGION: Lazy<Regex> = Lazy::new(|| {
    let mut source = regex::escape(PREFIX_PRE_HASH);
    source.push_str(r"(?P<hash>\w*)"); // maybe some hash
    source.extend(regex::escape(PREFIX_POST_HASH).drain(..));
    source.push_str(r"[\s\S]*?"); // everything non-greedy
    source.extend(regex::escape(SUFFIX).drain(..));

    Regex::new(&source).unwrap()
});

fn make_edit_offsets(contents: &str, force: bool) -> Option<(usize, usize, String)> {
    let parsed = parse(contents);
    if parsed.enums.is_empty() {
        // no enums to generate
        return None;
    }
    let mut hasher = DefaultHasher::new();
    parsed.hash(&mut hasher);
    ENUM_STRUCTURE_VERSION.hash(&mut hasher);
    CODE_GEN_VERSION.hash(&mut hasher);
    let hash_str = format!("{:x}", hasher.finish());
    let prefix: String = String::from(PREFIX_PRE_HASH) + &hash_str + PREFIX_POST_HASH;
    if !force && contents.contains(&prefix) {
        None
    } else {
        let mut to_write = prefix;
        to_write.extend(generate(parsed).drain(..));
        to_write.push_str(&SUFFIX);

        Some(
            if let Some(replace_at) = RE_ENUM_TS_REGION.find(&contents) {
                let (start, end) = (replace_at.start(), replace_at.end());
                (start, end, to_write)
            } else {
                let end = contents.len() - 1;
                (end, end, to_write)
            },
        )
    }
}

pub fn make_edit(contents: &str, force: bool) -> Option<(Position, Position, String)> {
    make_edit_offsets(&contents, force).map(|(start_offset, end_offset, to_insert)| {
        let mut str_pos = StringPositions::new(&contents);
        (
            str_pos
                .get_pos(start_offset)
                .expect("start replace has line column"),
            str_pos
                .get_pos(end_offset)
                .expect("end replace has line column"),
            to_insert,
        )
    })
}

pub fn rewrite(contents: &str, force: bool) -> Option<String> {
    make_edit_offsets(contents, force).map(|(start_offset, end_offset, mut content)| {
        let (before, _) = contents.split_at(start_offset);
        let (_, after) = contents.split_at(end_offset);
        let mut to_write = String::from(before);
        to_write.extend(content.drain(..));
        to_write.push_str(after);
        to_write
    })
}

pub fn rewrite_file<P: AsRef<Path>>(path: P, write: bool, force: bool) -> bool {
    let path_ref = path.as_ref();
    let file_contents =
        fs::read_to_string(&path_ref).expect("Something went wrong reading the file");
    if let Some(substitution) = rewrite(&file_contents, force) {
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
