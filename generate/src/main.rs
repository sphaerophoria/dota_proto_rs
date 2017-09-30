extern crate regex;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate protoc_rust;
extern crate pretty_env_logger;

mod errors {
    error_chain!{ }
}
use errors::*;

use std::fs::{File, OpenOptions};
use std::fs;
use std::ffi::OsString;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::io::Write;
use std::process::Command;

use regex::Regex;

lazy_static! {
    static ref DOTA_PROTO_ROOT: PathBuf = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../"));
    static ref GAMETRACKING_FOLDER: PathBuf = DOTA_PROTO_ROOT.join("GameTracking-Dota2");
    static ref PROTOBUF_FOLDER: PathBuf = GAMETRACKING_FOLDER.join("Protobufs");
    static ref CRATE_FOLDER: PathBuf = DOTA_PROTO_ROOT.join("generated");
    static ref PROTO_TMP: PathBuf = CRATE_FOLDER.join("dota_proto");
}

fn get_proto_paths() -> Result<Vec<PathBuf>>
{
    info!("{:?}", *PROTOBUF_FOLDER);
    let items = fs::read_dir(PROTOBUF_FOLDER.as_path())
        .chain_err(||"Failed to open")?
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .filter(|entry|  entry.path().extension() == Some(&OsString::from("proto")) )
        .map(|entry| entry.path())
        .collect();

    Ok(items)
}

fn get_dependencies(proto_path: &Path) -> Result<Vec<String>>
{
    let mut res = Vec::new();
    let file = BufReader::new(File::open(&proto_path)
        .chain_err(|| "Failed to open")?);

    lazy_static! {
        static ref IMPORT_REGEX: Regex = Regex::new("^import \"(.*)\"").unwrap();
    }

    for line in file.lines() {
        let line = line
            .chain_err(||"Failed to read file")?;

        for capture in IMPORT_REGEX.captures(&line) {
            let dep = &capture[1];
            if dep.contains("google/protobuf") {
                continue;
            }
            res.push(dep.to_string());
        }
    }
    Ok(res)
}

fn get_basename(proto_path: &Path) -> Result<String> {
    let basename: String = PathBuf::from(proto_path.file_name().unwrap())
        .file_stem()
        .ok_or("could not generate basename")?
        .to_str()
        .ok_or("Failed ot convert basename to str")?
        .into();

    Ok(basename)
}

fn get_mod_name(basename: &str) -> String {
    basename.replace(".", "_")
}

fn generate_subcrate(proto_path: &Path) -> Result<()>
{
    let dependencies = get_dependencies(proto_path)?;

    let basename = get_basename(proto_path)?;
    let mod_name = get_mod_name(&basename);

    let crate_dir = CRATE_FOLDER.join(&mod_name);

    info!("Crate dir: {:?}", crate_dir);

    fs::create_dir_all(&crate_dir)
        .chain_err(|| "Failed to create crate dir")?;

    let src_dir = crate_dir.join("src");
    fs::create_dir_all(&src_dir)
        .chain_err(|| "Failed to create src dir")?;

    let filename = PathBuf::from(&get_mod_name(&basename)).with_extension("rs");
    let from = PROTO_TMP.join(&filename);
    let to = src_dir.join(&filename);

    info!("{:?}, {:?}, {:?}, {:?}", PROTO_TMP.as_path(), filename, from, to);

    fs::rename(&from, &to)
        .chain_err(|| "Failed to rename proto generated file")?;

    info!("{:?}, {:?}", proto_path, get_dependencies(&proto_path)?);

    let mut cargo_toml = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(crate_dir.join("Cargo.toml"))
        .chain_err(||"Failed to open cargo.toml")?;

    write!(&mut cargo_toml,
        " \
        [package]\n\
        name = \"{0}\" \n\
        version = \"0.1.0\" \n\
        \n\
        [dependencies] \n\
        protobuf = \"1.4.1\"\n\
        ", mod_name).chain_err(|| "Failed to write to cargo.toml")?;

    let mut lib_rs = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(src_dir.join("lib.rs"))
        .chain_err(|| "Failed to open lib.rs")?;

    write!(lib_rs, "extern crate protobuf;\n")
        .chain_err(|| "Failed to write to lib.rs")?;

    for mut dependency in dependencies {
        let suffix = dependency.find(".proto").ok_or("Unexpected suffix")?;

        dependency.truncate(suffix);
        let dependency = dependency.replace(".", "_");

        write!(lib_rs, "extern crate {};\n", dependency)
            .chain_err(|| "Failed to add dependency to lib.rs")?;

        write!(cargo_toml, "{0} = {{ path = \"../{0}\" }}\n", dependency)
            .chain_err(|| "Failed to add dependency to Cargo.toml")?;
    }

    write!(lib_rs,
        " \
        mod {0};\n\
        pub use {0}::*;\n\
        ", mod_name)
        .chain_err(|| "Failed to write dependencies")?;

    Ok(())
}

fn generate_maincrate(proto_paths: &Vec<PathBuf>) -> Result<()> {
    lazy_static! {
        static ref MAINCRATE_PATH: PathBuf = DOTA_PROTO_ROOT.join("dota_proto_generated");
        static ref MAINCRATE_SRC_PATH: PathBuf = MAINCRATE_PATH.join("src");
    }

    fs::create_dir_all(MAINCRATE_PATH.as_path())
        .chain_err(|| "Failed to create maincrate directory")?;

    fs::create_dir_all(MAINCRATE_SRC_PATH.as_path())
        .chain_err(|| "Failed to create maincrate directory")?;

    let mut cargo_toml = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(MAINCRATE_PATH.join("Cargo.toml"))
        .chain_err(||"Failed to open maincrate cargo.toml")?;

    let mut lib_rs = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(MAINCRATE_SRC_PATH.join("lib.rs"))
        .chain_err(||"Failed to open maincrate lib.rs")?;

    write!(&mut cargo_toml,
        " \
        [package]\n\
        name = \"dota_proto_generated\" \n\
        version = \"0.1.0\" \n\
        \n\
        [dependencies] \n\
        ").chain_err(|| "Failed to write to cargo.toml")?;

    for proto_path in proto_paths {
        let mut proto_path = proto_path.file_name().ok_or("")?.to_str().ok_or("")?.to_string();
        let suffix = proto_path.find(".proto").ok_or("Unexpected suffix")?;

        proto_path.truncate(suffix);

        let proto_path = proto_path.replace(".", "_");
        write!(lib_rs, "\
        pub extern crate {0};\n\
        ", proto_path)
            .chain_err(|| "Failed to add dependency to lib.rs")?;

        write!(cargo_toml, "{0} = {{ path = \"../generated/{0}\" }}\n", proto_path)
            .chain_err(|| "Failed to add dependency to Cargo.toml")?;
    }

    Ok(())

}

fn update_repository() -> Result<()> {
    let git_dir_arg = format!("--git-dir={}", GAMETRACKING_FOLDER.to_string_lossy());

    let res = Command::new("git")
        .args(&[&git_dir_arg, "pull"])
        .status();

    if res.is_ok() {
        return Ok(());
    }

    Command::new("git")
        .args(&[&git_dir_arg, "clone", ""])
        .status()
        .chain_err(|| "Failed to clone")?;

    Ok(())
}

fn generate() -> Result<()> {
    pretty_env_logger::init()
        .chain_err(|| "Could not initialize logger")?;

    update_repository().chain_err(|| "Failed to update repository")?;

    fs::remove_dir_all(&*PROTO_TMP)
        .or_else(|e| if e.kind() == std::io::ErrorKind::NotFound { return Ok(());} else {return Err(e); })
        .chain_err(|| "Failed to remove proto dir")?;

    fs::remove_dir_all(&*CRATE_FOLDER)
        .or_else(|e| if e.kind() == std::io::ErrorKind::NotFound { return Ok(());} else {return Err(e); })
        .chain_err(|| "Failed to remove crate dir")?;

    let proto_paths = get_proto_paths()?;

    fs::create_dir_all(PROTO_TMP.as_path())
        .chain_err(|| "Failed to create proto output directory")?;

    let protos = proto_paths
        .iter()
        .map(|path| path.clone().into_os_string().into_string().unwrap())
        .collect::<Vec<_>>();

    let proto_strs = protos.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    for proto in &proto_strs {
        protoc_rust::run(protoc_rust::Args{
            out_dir: PROTO_TMP.as_os_str().to_str().ok_or("Failed to convert path to str")?,
            input: &[proto],
            includes: &[PROTOBUF_FOLDER.as_os_str().to_str().ok_or("Failed to convert path to str")?, "/usr/include"],
        }).chain_err(||"Failed to generate protos")?;
    }

    let proto_paths = get_proto_paths()?;

    for proto_path in &proto_paths {
        generate_subcrate(proto_path)
            .chain_err(|| "Failed to generate subcrate")?;
    }

    generate_maincrate(&proto_paths)?;
    Ok(())
}

quick_main!(generate);
