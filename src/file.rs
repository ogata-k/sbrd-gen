use std::fs::File;
use std::path::PathBuf;
use std::{env, io};

const SCHEMA_FILE_PATH_KEY: &str = "SBRD_SCHEMA_FILE";

pub fn set_schema_file_path<P>(schema_filepath: P)
where
    P: Into<PathBuf>,
{
    let path = schema_filepath.into();
    env::set_var(SCHEMA_FILE_PATH_KEY, path);
}

pub(crate) fn open_sbrd_file<P>(filepath: P) -> io::Result<File>
where
    P: Into<PathBuf>,
{
    match env::var(SCHEMA_FILE_PATH_KEY) {
        Ok(schema_filepath) => {
            let mut _filepath = PathBuf::from(schema_filepath);
            _filepath.set_file_name(filepath.into());
            File::open(_filepath)
        }
        Err(_) => File::open(filepath.into()),
    }
}
