use std::{
    fs,
    io::{self, Read, Write},
    process,
};

use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde_json::{Value, json};
use zip::ZipArchive;

const PYTHON_DEP_ZIP_FILE: &[u8] = include_bytes!("../py-deps.zip");
const PYTHON_DEPS_EXTACTED_DIR: &str = "pyhanko-deps";
const TEST_SCRIPT: &str = include_str!("../test.py");
const TEST_DOC: &[u8] = include_bytes!("../test.pdf");
const TEST_DOC_OUTPUT: &str = "test_out.pdf";

pub fn init_python_env() {
    if !fs::exists(PYTHON_DEPS_EXTACTED_DIR).unwrap() {
        fs::create_dir(PYTHON_DEPS_EXTACTED_DIR).unwrap()
    }

    // Extract the python dependencies
    let zip_file = io::Cursor::new(PYTHON_DEP_ZIP_FILE);
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    zip_archive.extract(&PYTHON_DEPS_EXTACTED_DIR).unwrap();
}

// fn main() -> Result<(), Error> {
//     init_python_env();
//
//     let child = process::Command::new("python3")
//         .args(&["-c", TEST_SCRIPT])
//         .stdin(process::Stdio::piped())
//         .stdout(process::Stdio::piped())
//         .spawn()
//         .unwrap();
//
//     let mut child_in = child.stdin.unwrap();
//     child_in.write_all(TEST_DOC).unwrap();
//     drop(child_in);
//
//     let mut child_out = child.stdout.unwrap();
//     let mut out_buf = Vec::new();
//     child_out.read_to_end(&mut out_buf).unwrap();
//
//     let mut output_file = fs::OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .create(true)
//         .write(true)
//         .open(TEST_DOC_OUTPUT)
//         .unwrap();
//
//     output_file.write_all(&mut out_buf).unwrap();
//
//     Ok(())
// }

fn do_stuff() -> Vec<u8> {
    init_python_env();

    let child = process::Command::new("python3")
        .args(&["-c", TEST_SCRIPT])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut child_in = child.stdin.unwrap();
    child_in.write_all(TEST_DOC).unwrap();
    drop(child_in);

    let mut child_out = child.stdout.unwrap();
    let mut out_buf = Vec::new();
    child_out.read_to_end(&mut out_buf).unwrap();

    out_buf
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_python_env();

    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let out = do_stuff();
    let pdf = base64::encode(out);

    eprint!("Nothing wrong happened");
    Ok(json!({ "pdf": pdf }))
}
