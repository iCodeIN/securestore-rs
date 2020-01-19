#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorKind {
    /// The key did not meet the requirements for a valid keyfile.
    InvalidKeyfile,
    /// May be caused by using the wrong key or attempting to load ciphertext
    /// that has been tampered with.
    DecryptionFailure,
    SecretNotFound,
    UnsupportedVaultVersion,
    IoError,
    /// An error occurred during the (de)serialization process.
    InvalidStore,
}

#[derive(Debug)]
pub struct Error {
    inner: Option<Box<dyn std::error::Error + 'static + Send>>,
    kind: ErrorKind,
    message: Option<String>,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool {
        self.kind == rhs.kind && self.inner.is_some() == self.inner.is_some()
    }
}

impl std::convert::From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            kind,
            inner: None,
            message: None,
        }
    }
}

impl<R> std::convert::Into<Result<R, Error>> for Error {
    fn into(self) -> Result<R, Error> {
        Err(self)
    }
}

impl<R> std::convert::Into<Result<R, Error>> for ErrorKind {
    fn into(self) -> Result<R, Error> {
        Err(self.into())
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            inner: Some(Box::new(e)),
            kind: ErrorKind::IoError,
            message: None,
        }
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error {
            inner: Some(Box::new(e)),
            kind: ErrorKind::InvalidStore,
            message: None,
        }
    }
}

impl std::convert::From<openssl::error::ErrorStack> for Error {
    fn from(e: openssl::error::ErrorStack) -> Error {
        Error {
            inner: Some(Box::new(e)),
            kind: ErrorKind::DecryptionFailure,
            message: None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.kind() {
            ErrorKind::InvalidKeyfile => "An invalid key file was supplied",
            ErrorKind::DecryptionFailure => "There was an error decrypting the secrets store. Check the password or key file and verify the store has not been tampered with",
            ErrorKind::SecretNotFound => "No secret was found with the specified name",
            ErrorKind::UnsupportedVaultVersion => "An attempt was made to open a vault with an unsupported version",
            ErrorKind::IoError => "An IO error occurred",
            ErrorKind::InvalidStore => "The contents of the store did not match what was expected",
        };

        match &self.inner {
            Some(inner) => write!(fmt, "{}: {}", s, inner),
            None => write!(fmt, "{}.", s),
        }
    }
}

impl std::error::Error for Error {}