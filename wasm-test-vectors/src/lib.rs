//! A bundle of polis WASM test vectors

use include_dir::{include_dir, Dir};
use std::collections::BTreeMap;

static WASM_PAYLOAD_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/wasm/target/wasm-payloads");

/// Return a mapping of `{ payload_name -> &[u8] }`
///
/// The `payload_name` is the filename without a `".wasm"` suffix.
pub fn wasm_vectors() -> BTreeMap<&'static str, &'static [u8]> {
    let mut map = BTreeMap::default();

    for file in WASM_PAYLOAD_DIR.files() {
        let name = file
            .path()
            .file_name()
            .unwrap_or_else(|| panic!("no filename: {:?}", file))
            .to_str()
            .unwrap_or_else(|| panic!("non-utf8 filename: {:?}", file))
            .strip_suffix(".wasm")
            .unwrap_or_else(|| panic!("missing \".wasm\" suffix: {:?}", file));
        let contents = file.contents();

        if let Some(oldcontents) = map.insert(name, contents) {
            panic!(
                "Duplicate entry for {:?}: old {:?} bytes, new {:?} bytes",
                name,
                oldcontents.len(),
                contents.len()
            );
        }
    }

    map
}

#[test]
fn smoke_test() {
    let map = wasm_vectors();

    eprintln!(
        "Entries:\n{}",
        map.iter()
            .map(|(k, v)| format!("  {:?}: {} bytes", k, v.len()))
            .collect::<Vec<String>>()
            .join(",\n")
    );
    assert!(!map.is_empty());
}
