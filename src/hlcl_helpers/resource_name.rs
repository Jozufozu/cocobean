use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NameError {
    InvalidChar(usize, char),
    MultipleColons { first: usize, second: usize },
    MissingColon,
    MissingNamespace,
    MissingPath,
    TooLong,
}

impl Display for NameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "error while creating resource name:\n  ")?;
        match self {
            NameError::InvalidChar(loc, char) => {
                write!(f, "invalid char '{}' at index {}", char, loc)
            }
            NameError::MultipleColons { first, second } => write!(
                f,
                "colon at index {}, but there is already one at index {}",
                first, second
            ),
            NameError::MissingColon => write!(f, "missing colon"),
            NameError::MissingNamespace => write!(f, "missing namespace"),
            NameError::MissingPath => write!(f, "missing path"),
            NameError::TooLong => write!(f, "must be at most 65535 characters"),
        }
    }
}

impl Error for NameError {}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ResourceName {
    data: String,
    colon: u16,
}

impl ResourceName {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.data.as_str()
    }

    #[inline]
    pub fn path(&self) -> &str {
        &self.data[(self.colon as usize + 1)..]
    }

    #[inline]
    pub fn namespace(&self) -> &str {
        &self.data[..(self.colon as usize)]
    }

    fn check_str<S: AsRef<str>>(str: &S) -> Result<usize, NameError> {
        let str = str.as_ref();

        let len = str.len();

        if len > u16::max_value() as usize {
            return Err(NameError::TooLong);
        }

        let mut colon = None;

        for (i, char) in str.char_indices() {
            match char {
                ':' => {
                    if let Some(first) = colon {
                        return Err(NameError::MultipleColons { first, second: i });
                    } else if i == 0 {
                        return Err(NameError::MissingNamespace);
                    } else {
                        colon = Some(i);
                    }
                }
                'a'..='z' | '0'..='9' | '_' | '-' => (),
                '/' | '.' if colon.is_some() => (),
                _ => return Err(NameError::InvalidChar(i, char)),
            }
        }

        let colon = colon.ok_or(NameError::MissingColon)?;

        if colon == len - 1 {
            Err(NameError::MissingPath)
        } else {
            Ok(colon)
        }
    }
}

impl TryFrom<String> for ResourceName {
    type Error = NameError;

    fn try_from(data: String) -> Result<Self, NameError> {
        let colon = ResourceName::check_str(&data)?;

        Ok(ResourceName {
            data,
            colon: colon as u16,
        })
    }
}

impl AsRef<str> for ResourceName {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.data.as_str()
    }
}

#[cfg(test)]
pub mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::resource_name::{NameError, ResourceName};

    #[test]
    fn colon_placement() {
        let name = ResourceName::try_from("minecraft:test".to_owned()).unwrap();

        assert_eq!(name.colon, 9);
    }

    #[test]
    fn file_path() {
        ResourceName::try_from("testing:test/test1.json".to_owned()).unwrap();
    }

    #[test]
    fn splitting() {
        let name = ResourceName::try_from("minecraft:test".to_owned()).unwrap();

        assert_eq!("test", name.path());
        assert_eq!("minecraft", name.namespace());
    }

    #[test]
    fn invalid_char() {
        let error = ResourceName::try_from("testing: bad stuff".to_owned()).unwrap_err();

        assert_eq!(error, NameError::InvalidChar(8, ' '));
    }

    #[test]
    fn missing_namespace() {
        let error = ResourceName::try_from(":bad_stuff".to_owned()).unwrap_err();

        assert_eq!(error, NameError::MissingNamespace);
    }

    #[test]
    fn missing_path() {
        let error = ResourceName::try_from("oh_no:".to_owned()).unwrap_err();

        assert_eq!(error, NameError::MissingPath);
    }

    #[test]
    fn missing_colon() {
        let error = ResourceName::try_from("oh_no_this_fails".to_owned()).unwrap_err();

        assert_eq!(error, NameError::MissingColon);
    }

    #[test]
    fn slash_in_namespace() {
        let error = ResourceName::try_from("test/path:file".to_owned()).unwrap_err();

        assert_eq!(error, NameError::InvalidChar(4, '/'));
    }

    #[test]
    fn dot_in_namespace() {
        let error = ResourceName::try_from("test.path:file".to_owned()).unwrap_err();

        assert_eq!(error, NameError::InvalidChar(4, '.'));
    }
}
