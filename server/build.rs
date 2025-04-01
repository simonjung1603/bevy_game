use std::env::{self, current_dir};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("../../../game");
    if !fs::exists(&dest_path).unwrap(){
        fs::create_dir(&dest_path).unwrap();
    }
    let dest_path = dest_path.canonicalize().unwrap();

    fs::copy("web/index.html", dest_path.join("index.html")).unwrap();
    let status = Command::new("cargo").args(["build", "--release", "--bin", "bevy_game", "--target", "wasm32-unknown-unknown"]).current_dir(current_dir().unwrap().parent().unwrap().join("game")).status().unwrap();
    if !status.success(){
        panic!("Building the game failed.");
    }
    let status = Command::new("wasm-bindgen").args(["--out-dir", format!("{}", dest_path.display()).as_str(), "--target", "web", "../target/wasm32-unknown-unknown/release/bevy_game.wasm"]).status().unwrap();
    if !status.success(){
        panic!("wasm-bindgen failed.");
    }
    println!("cargo::rerun-if-changed=web");
    println!("cargo::rerun-if-changed=../game");
}