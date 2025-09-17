use std::{
    fs::{self, exists},
    io::{self, BufWriter, Read, Write},
    process::Command,
};

use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};

const PY_DEPS_PATH: &str = "py-deps";
const PY_INSTALL_DEPS: &str = "pyHanko[pkcs11,image-support,opentype,qr]";
const PY_DEPS_ZIP: &str = "py-deps.zip";

fn main() {
    if fs::exists(PY_DEPS_ZIP).unwrap() {
        return;
    }
    // Install pyhanko in dependency directory
    fs::create_dir_all(PY_DEPS_PATH).expect("Failed to create python dependency directory");
    let mut pip_process = Command::new(format!("pip"))
        .args(&[
            "install",
            PY_INSTALL_DEPS,
            "--target",
            PY_DEPS_PATH,
            "--upgrade",
            "--platform",
            "manylinux2014_x86_64",
            "--only-binary=:all:",
        ])
        .spawn()
        .expect("Failed to spawn pip");

    let pip_exit_status = pip_process
        .wait()
        .expect("Failed to get exit status of pip process");

    if !pip_exit_status.success() {
        panic!("pip failed with exit status: {pip_exit_status}");
    }

    // Zip the dependency directory
    let zip_file = BufWriter::new(
        fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(PY_DEPS_ZIP)
            .expect("Failed to open PY_DEPS_ZIP file"),
    );

    let mut zip_writer = ZipWriter::new(zip_file);
    let mut dep_file_buf = Vec::new();
    for entry in WalkDir::new(PY_DEPS_PATH) {
        let entry = entry.expect("Failed to read directory entry");
        assert!(!entry.path_is_symlink());

        if entry.file_type().is_dir() {
            continue;
        }

        zip_writer
            .start_file_from_path(
                entry.path().strip_prefix(PY_DEPS_PATH).unwrap(),
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated),
            )
            .expect("failed to create file in zip");

        dep_file_buf.clear();
        let mut dep_file = fs::OpenOptions::new()
            .read(true)
            .open(entry.path())
            .expect("failed open file");
        dep_file
            .read_to_end(&mut dep_file_buf)
            .expect("Failed to read dep file");

        zip_writer
            .write_all(&mut dep_file_buf)
            .expect("Failed to zip dep file");
    }
}
