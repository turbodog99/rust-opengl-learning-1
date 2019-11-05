use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "I/O error")]
  Io(#[cause] io::Error),
  #[fail(display = "Failed to read CString from file that contains 0")]
  FileContainsNil,
  #[fail(display = "Failed to get executable path")]
  FailedToGetExePath,
}

impl From<io::Error> for Error {
  fn from(other: io::Error) -> Self {
    Error::Io(other)
  }
}

trait ExePaths {
  fn exe_path() -> Result<PathBuf, Error>;

  fn relative_exe_path(rel_path: &Path) -> Result<PathBuf, Error> {
    let exe_path = Self::exe_path()?;
    let exe_dir = exe_path.parent().unwrap();
    let full_path = exe_dir.join(rel_path);
    Ok(full_path)
  }
}

pub struct Resources {
  root_path: PathBuf,
}

impl Resources {
  pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
    Ok(Resources {
      root_path: Self::relative_exe_path(rel_path).unwrap(),
    })
  }

  pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
    let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;

    // allocate buffer of the same size as file
    let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
    file.read_to_end(&mut buffer)?;

    // check for nul byte
    if buffer.iter().find(|i| **i == 0).is_some() {
      return Err(Error::FileContainsNil);
    }

    Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
  }
}

impl ExePaths for Resources {
  fn exe_path() -> Result<PathBuf, Error> {
    ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)
  }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
  let mut path: PathBuf = root_dir.into();

  for part in location.split("/") {
    path = path.join(part);
  }

  path
}

#[cfg(test)]
mod tests {
  use super::*;

  struct ExePathsTest {}

  impl ExePaths for ExePathsTest {
    fn exe_path() -> Result<PathBuf, Error> {
      Ok(Path::new("/test/test.exe").to_path_buf())
    }
  }

  #[test]
  fn relative_exe_path_test() {
    assert_eq!(
      ExePathsTest::relative_exe_path(Path::new("assets")).unwrap(),
      Path::new("/test/assets").to_path_buf()
    );
  }
}
