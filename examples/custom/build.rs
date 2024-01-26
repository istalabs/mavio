use std::env::var;
use std::path::Path;

use mavspec::rust::gen::BuildHelper;

fn main() {
    let included_dialects = ["common"];
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sources = [manifest_dir.join("message_definitions")];
    let destination = Path::new(&var("OUT_DIR").unwrap()).join("mavlink");
    let manifest_path = manifest_dir.join("Cargo.toml");

    BuildHelper::builder(destination)
        .set_sources(&sources)
        .set_include_dialects(&included_dialects)
        .set_manifest_path(&manifest_path)
        .generate()
        .unwrap();
}
