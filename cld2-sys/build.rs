extern crate cc;
extern crate toml;

use std::borrow::ToOwned;
use std::collections::HashSet;
use std::env;
use std::fs::{File, read_dir};
use std::io::Read;
use std::path::{Path, PathBuf};

// Fetch the 'package.exclude' list from our Cargo.toml file.  We'll
// use this to decide what sources to admit.
fn get_excluded_sources(manifest: &Path) -> HashSet<String> {
    let mut text = String::new();
    File::open(manifest)
        .unwrap()
        .read_to_string(&mut text)
        .unwrap();
    let toml = text.parse::<toml::Table>().unwrap();
    let package = toml.get("package").unwrap().as_table().unwrap();
    let exclude = package.get("exclude").unwrap().as_array().unwrap();
    exclude
        .iter()
        .map(|e| {
            let str = e.as_str().unwrap();
            Path::new(str)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

// Get all the *.cc files in path that aren't excluded.
fn get_cc_files(dir: &Path, excluded: &HashSet<String>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|p| {
            let filename = p.file_name().unwrap().to_str().unwrap();
            filename.ends_with(".cc") && !excluded.contains(filename)
        })
        .map(|p| p.to_owned())
        .collect();
    files.sort();
    files
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&manifest_dir);

    // Decide what sources to build.
    let excluded = get_excluded_sources(&src.join("Cargo.toml"));
    let mut sources = get_cc_files(&src.join("cld2").join("internal"), &excluded);
    sources.push(src.join("src").join("wrapper.cpp"));

    // Convert the sources back to relative path &str values.
    // TODO: This required the unstable relative_from function.
    //let rel_sources: Vec<PathBuf> = sources.iter().map(|p| {
    //    p.relative_from(&src).unwrap().to_owned()
    //}).collect();

    let mut config = cc::Build::new();
    config.cpp(true);
    config.flag("-std=c++03");
    config.flag_if_supported("-Wno-c++11-narrowing");
    config.include(src.join("cld2/public"));
    config.include(src.join("cld2/internal"));
    for f in sources.iter() {
        config.file(f);
    }
    config.compile("cld2");
}
