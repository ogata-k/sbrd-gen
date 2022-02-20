use std::fs::File;
use std::path::PathBuf;
use std::{env, io};

const SCHEME_FILE_PATH_KEY: &str = "SBRD_SCHEME_FILE";

pub fn set_scheme_file_path<P>(scheme_filepath: P)
where
    P: Into<PathBuf>,
{
    let path = scheme_filepath.into();
    env::set_var(SCHEME_FILE_PATH_KEY, path);
}

pub(crate) fn open_sbrd_file<P>(filepath: P) -> io::Result<File>
where
    P: Into<PathBuf>,
{
    match env::var(SCHEME_FILE_PATH_KEY) {
        Ok(scheme_filepath) => {
            let mut _filepath = PathBuf::from(scheme_filepath);
            _filepath.set_file_name(filepath.into());
            File::open(_filepath)
        }
        Err(_) => File::open(filepath.into()),
    }
}
