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
            "cs_air_link",
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
        if var("CARGO_CFG_DOCTEST").is_ok() {
            included_dialects.push("minimal".to_string())
        }

        included_dialects
    };

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sources = [manifest_dir.join("message_definitions")];
    let destination = Path::new(&var("OUT_DIR").unwrap()).join("mavlink");
    let serde_feature_enabled = var("CARGO_FEATURE_SERDE").is_ok();

    BuildHelper::builder(destination)
        .set_sources(&sources)
        .set_include_dialects(&included_dialects)
        .set_serde(serde_feature_enabled)
        .generate()
        .unwrap();
}
