//! Module for file operating

use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, io};

/// Constant value's key of the specified schema file
const SCHEMA_FILE_PATH_KEY: &str = "SBRD_SCHEMA_FILE";

/// Make the environment remember the schema file path.
/// The file path specified in the schema is interpreted starting from the file path of the stored schema if it exists, otherwise it is interpreted starting from the path of the command when the command was executed.
/// Normally, the schema file path is remembered when you run a CLI command.
pub fn set_schema_file_path<P>(schema_filepath: P)
where
    P: Into<PathBuf>,
{
    let path = schema_filepath.into();
    env::set_var(SCHEMA_FILE_PATH_KEY, path);
}

/// The file path specified in the schema is interpreted starting from the file path of the stored schema if it exists, otherwise it is interpreted starting from the path of the command when the command was executed.
pub(crate) fn open_sbrd_file(filepath: &Path) -> io::Result<File> {
    match env::var(SCHEMA_FILE_PATH_KEY) {
        Ok(schema_filepath) => {
            let mut _filepath = PathBuf::from(schema_filepath);
            _filepath.set_file_name(filepath);
            File::open(_filepath)
        }
        Err(_) => File::open(filepath),
    }
}
