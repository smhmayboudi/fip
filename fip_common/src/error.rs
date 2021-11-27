//! TODO: documentation

/// TODO: documentation
#[derive(Debug)]
#[non_exhaustive]
pub struct Error {
    error_kind: ErrorKind,
    span_trace: tracing_error::SpanTrace,
}

/// TODO: documentation
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ErrorKind {
    /// TODO: documentation
    #[error(transparent)]
    KafkaError(#[from] rdkafka::error::KafkaError),
    /// TODO: documentation
    #[error(transparent)]
    HttpInvalidUri(#[from] http::uri::InvalidUri),
    /// TODO: documentation
    #[error(transparent)]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    /// TODO: documentation
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    /// TODO: documentation
    #[error(transparent)]
    TonicMetadataToStrError(#[from] tonic::metadata::errors::ToStrError),
    /// TODO: documentation
    #[error(transparent)]
    TonicTransportError(#[from] tonic::transport::Error),
}

/// TODO: documentation
#[allow(clippy::match_same_arms)]
impl From<Error> for tonic::Status {
    /// TODO: documentation
    fn from(err: Error) -> Self {
        match err.error_kind {
            ErrorKind::KafkaError(err) => Self::unknown(format!("{:?}", err)),
            ErrorKind::HttpInvalidUri(err) => Self::unknown(format!("{:?}", err)),
            ErrorKind::JsonWebTokenError(err) => Self::unknown(format!("{:?}", err)),
            ErrorKind::SqlxError(err) => Self::unknown(format!("{:?}", err)),
            ErrorKind::TonicMetadataToStrError(err) => Self::unknown(format!("{:?}", err)),
            ErrorKind::TonicTransportError(err) => Self::unknown(format!("{:?}", err)),
        }
    }
}

impl<E> From<E> for Error
where
    ErrorKind: From<E>,
{
    /// TODO: documentation
    fn from(err: E) -> Self {
        Self {
            error_kind: ErrorKind::from(err),
            span_trace: tracing_error::SpanTrace::capture(),
        }
    }
}

/// TODO: documentation
impl std::fmt::Display for Error {
    /// TODO: documentation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\nerror_kind: {:?}", self.error_kind)?;
        write!(f, "\n\nspan_trace: {:?}", self.span_trace)
    }
}
