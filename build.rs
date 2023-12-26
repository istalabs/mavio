use std::env::var;
use std::path::Path;

use mavspec::rust::gen::BuildHelper;

fn main() {
    let included_dialects = {
        let mut included_dialects: Vec<String> = Default::default();

        let dialects = vec![
            "all",
            "ardupilotmega",
            "asluav",
            "avssuas",
            "common",
            "csairlink",
            "cubepilot",
            "development",
            "icarous",
            "matrixpilot",
            "minimal",
            "paparazzi",
            "standard",
            "ualberta",
            "uavionix",
        ];
        for dialect in dialects {
            if var(format!("CARGO_FEATURE_{}", dialect.to_ascii_uppercase())).is_ok() {
                included_dialects.push(dialect.to_string())
            }
        }

        included_dialects
    };

    let sources = ["./message_definitions/standard"];
    let destination = Path::new(&var("OUT_DIR").unwrap()).join("mavlink");
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let serde_feature_enabled = var("CARGO_FEATURE_SERDE").is_ok();

    BuildHelper::builder(&destination)
        .set_sources(&sources)
        .set_manifest_path(&manifest_path)
        .set_serde(serde_feature_enabled)
        .set_include_dialects(&included_dialects)
        .generate()
        .unwrap();
}
