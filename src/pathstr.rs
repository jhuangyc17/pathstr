use std::ffi::OsStr;
use std::path::Path;

pub trait PathStrExt {
  fn try_to_str(&self) -> Result<&str, InvalidUtf8Error>;
}

pub trait OptionPathStrExt {
  fn try_to_str(&self) -> Result<Option<&str>, InvalidUtf8Error>;
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid utf-8 encoding: {0}")]
pub struct InvalidUtf8Error(pub String);

impl PathStrExt for Path {
  fn try_to_str(&self) -> Result<&str, InvalidUtf8Error> {
    match self.to_str() {
      None => Err(InvalidUtf8Error(format!("{}", self.display()))),
      Some(s) => Ok(s),
    }
  }
}

impl OptionPathStrExt for Option<&OsStr> {
  fn try_to_str(&self) -> Result<Option<&str>, InvalidUtf8Error> {
    match self {
      None => Ok(None),
      Some(oss) => match oss.to_str() {
        None => Err(InvalidUtf8Error(format!("{}", oss.display()))),
        Some(s) => Ok(Some(s)),
      },
    }
  }
}

impl OptionPathStrExt for Option<&Path> {
  fn try_to_str(&self) -> Result<Option<&str>, InvalidUtf8Error> {
    match self {
      None => Ok(None),
      Some(p) => match p.to_str() {
        None => Err(InvalidUtf8Error(format!("{}", p.display()))),
        Some(s) => Ok(Some(s)),
      },
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use assert_matches::assert_matches;
  use std::ffi::OsString;
  use std::os::unix::ffi::OsStringExt;
  use std::path::PathBuf;

  #[test]
  fn should_handle_valid_utf8() {
    let p = PathBuf::from("abc/def/ghi.xyz");

    assert_matches!(p.try_to_str(), Ok(v) => {
      assert_eq!(v, "abc/def/ghi.xyz");
    });
    assert_matches!(p.parent().try_to_str(), Ok(v) => {
      assert_eq!(v, Some("abc/def"));
    });

    assert_matches!(p.file_name().try_to_str(), Ok(v) => {
      assert_eq!(v, Some("ghi.xyz"));
    });
    assert_matches!(p.file_stem().try_to_str(), Ok(v) => {
      assert_eq!(v, Some("ghi"));
    });
    assert_matches!(p.extension().try_to_str(), Ok(v) => {
      assert_eq!(v, Some("xyz"));
    });
  }

  #[test]
  fn should_handle_invalid_utf8() {
    let invalid_path = OsString::from_vec(vec![0xF0, 0x90, 0x80]);
    let invalid_path_display: String = format!("{}", invalid_path.display());

    let p = PathBuf::from(invalid_path);
    assert_matches!(p.try_to_str(), Err(e) => {
      assert_eq!(e.0, invalid_path_display);
    });
  }
}
