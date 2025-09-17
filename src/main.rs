use std::{fs, io};

use lambda_runtime::{Error, LambdaEvent, service_fn};
use pyo3::Python;
use serde_json::{Value, json};
use zip::ZipArchive;

const PYTHON_DEP_ZIP_FILE: &[u8] = include_bytes!("../py-deps.zip");
const PYTHON_DEPS_EXTACTED_DIR: &str = "pyhanko-deps";
const TEST_SCRIPT: &[u8] = include_bytes!("../test.py");

pub fn init_python_env() {
    // Create temp dir for the python dependencies
    // let mut temp_dir = env::temp_dir();
    // temp_dir.push(PYTHON_DEPS_EXTACTED_DIR);
    // fs::create_dir(temp_dir).anyerr()?;

    if !fs::exists(PYTHON_DEPS_EXTACTED_DIR).unwrap() {
        fs::create_dir(PYTHON_DEPS_EXTACTED_DIR).unwrap()
    }

    // Extract the python dependencies
    let zip_file = io::Cursor::new(PYTHON_DEP_ZIP_FILE);
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    zip_archive.extract(&PYTHON_DEPS_EXTACTED_DIR).unwrap();

    // Set PYTHONPATH environment variable to point at the extracted dependency directory
    // unsafe { env::set_var("PYTHONPATH", temp_dir) };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_python_env();

    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let result =
        Python::attach(|py| py.run(&std::ffi::CString::new(TEST_SCRIPT).unwrap(), None, None));

    eprint!("Nothing wrong happened");
    Ok(json!({ "message": format!("Result: {result:#?}") }))
}
