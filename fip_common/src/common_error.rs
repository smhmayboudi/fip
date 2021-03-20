#[derive(Debug)]
#[non_exhaustive]
pub struct CommonError {
    common_error_kind: CommonErrorKind,
    span_trace: tracing_error::SpanTrace,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CommonErrorKind {
    #[error(transparent)]
    KafkaError(#[from] rdkafka::error::KafkaError),

    #[error(transparent)]
    HttpInvalidUri(#[from] http::uri::InvalidUri),

    #[error(transparent)]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    TonicMetadataToStrError(#[from] tonic::metadata::errors::ToStrError),

    #[error(transparent)]
    TonicTransportError(#[from] tonic::transport::Error),
}

impl From<CommonError> for tonic::Status {
    fn from(common_error: CommonError) -> tonic::Status {
        match common_error.common_error_kind {
            CommonErrorKind::KafkaError(err) => tonic::Status::unavailable(format!("{:?}", err)),
            CommonErrorKind::HttpInvalidUri(err) => {
                tonic::Status::unavailable(format!("{:?}", err))
            }
            CommonErrorKind::JsonWebTokenError(err) => {
                tonic::Status::unauthenticated(format!("{:?}", err))
            }
            CommonErrorKind::SqlxError(err) => tonic::Status::unavailable(format!("{:?}", err)),
            CommonErrorKind::TonicMetadataToStrError(err) => {
                tonic::Status::unavailable(format!("{:?}", err))
            }
            CommonErrorKind::TonicTransportError(err) => {
                tonic::Status::unavailable(format!("{:?}", err))
            }
        }
    }
}

impl<E> From<E> for CommonError
where
    CommonErrorKind: From<E>,
{
    fn from(error: E) -> Self {
        Self {
            common_error_kind: CommonErrorKind::from(error),
            span_trace: tracing_error::SpanTrace::capture(),
        }
    }
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\ncommon_error_kind:\n{}", self.common_error_kind)?;
        write!(f, "\n\nspan_trace: {}", self.span_trace)
    }
}
