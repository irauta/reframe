
use std::error::Error;
use std::fmt::{self, Display};
use regl::ReglError;

#[derive(Debug)]
pub enum ReframeError {
    TooSmallBufferError,
    AttributeMappingError,
    ReglError(ReglError),
}

impl From<ReglError> for ReframeError {
    fn from(error: ReglError) -> ReframeError {
        ReframeError::ReglError(error)
    }
}

impl Display for ReframeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ReframeError {
    fn description(&self) -> &str {
        match *self {
            ReframeError::TooSmallBufferError =>
                "Byte buffer was not large enough to be mapped into a uniform type",
            ReframeError::AttributeMappingError =>
                "Could not find attribute index for named attribute",
            ReframeError::ReglError(ref regl_error) => regl_error.description(),
        }
    }
}
