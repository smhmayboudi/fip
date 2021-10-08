use fip_common::common_error::CommonError;
use tonic::Request;

/// TODO: documentation
pub trait Sub {
    /// TODO: documentation
    fn sub<T>(request: &Request<T>) -> Result<&str, CommonError> {
        request
            .metadata()
            .get("sub")
            .unwrap()
            .to_str()
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err.into()
            })
    }
}
