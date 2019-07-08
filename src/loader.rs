use super::ast;
use super::parser;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use super::name::Name;
use super::project::Config;


pub enum Module {
    C(PathBuf),
    ZZ(ast::Module),
}

pub fn load(
    modules:       &mut HashMap<Name, Module>,
    artifact_name: &Name,
    src:          &Path
) {
    for entry in std::fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path  = entry.path();
        if path.is_file() {
            let ext = path.extension().map(|v|v.to_str()).expect(&format!("invalid file name {:?}", path));
            match ext {
                Some("h") => {
                    let mut name = artifact_name.clone();
                    name.push(path.file_stem().unwrap().to_string_lossy().to_string());
                    modules.insert(name, Module::C(path.into()));
                },
                Some("zz") => {
                    let mut m = parser::parse(&path);
                    m.name = artifact_name.clone();
                    m.name.push(path.file_stem().unwrap().to_string_lossy().to_string());
                    modules.insert(m.name.clone(), Module::ZZ(m));
                },
                _ => {},
            }
        }
    }
}
