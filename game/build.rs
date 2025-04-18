use quick_xml::de as xml;
use serde::Deserialize;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct TextureAtlas {
    #[serde(rename = "SubTexture")]
    subtextures: Vec<SubTexture>,
}

#[derive(Debug, Deserialize)]
struct SubTexture {
    #[serde(rename = "@name")]
    name: String,
}

fn main() {
    let assets_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");

    let out_dir = assets_dir.join("generated");
    fs::create_dir_all(out_dir.clone())
        .expect("Could not create directory for generated asset indices.");

    let codegen_path = out_dir.join("spritesheet_asset_indices.rs");
    let mut file = BufWriter::new(File::create(&codegen_path).unwrap());

    let in_dir = assets_dir.join("images");
    println!("cargo:rerun-if-changed={}", in_dir.display());

    // Write the module documentation
    write!(
        &mut file,
        "// Auto-generated sprite indices\n// Each constant represents the index of a sprite in the texture atlas\n\n"
    ).unwrap();

    // Process all XML files in the images directory
    if let Ok(entries) = fs::read_dir(&in_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "xml") {
                println!("cargo:rerun-if-changed={}", path.display());
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let module_name = file_name
                    .replace(".xml", "")
                    .replace("-", "_")
                    .replace(" ", "_")
                    .to_lowercase();

                // Start a new module for this spritesheet
                writeln!(
                    &mut file,
                    "// Indices for: {:?}",
                    path.file_name().unwrap()
                )
                .unwrap();

                write!(&mut file, "#[allow(unused)]\npub mod {} {{\n", module_name).unwrap();

                let mut index = 0;
                let content = fs::read_to_string(&path).expect("Could not read content.");
                let atlas =
                    xml::from_str::<TextureAtlas>(&content).expect("Could not deserialize xml.");

                for subtexture in atlas.subtextures {
                    // Convert the sprite name to a valid Rust constant name
                    let const_name = subtexture
                        .name
                        .replace(".png", "")
                        .replace("-", "_")
                        .replace(" ", "_")
                        .to_uppercase();

                    // Write the constant definition
                    writeln!(
                        &mut file,
                        "    pub const {}: usize = {};",
                        const_name, index
                    )
                    .unwrap();

                    index += 1;
                }

                // Close the module
                write!(&mut file, "}}\n\n").unwrap();
            }
        }
    }
}
