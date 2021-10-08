use fip_common::error::Error;
use tonic::Request;

/// TODO: documentation
pub trait Sub {
    /// TODO: documentation
    ///
    /// # Errors
    /// TODO: documentation errors
    ///
    /// # Panics
    /// TODO: documentation panics
    fn sub<T>(request: &Request<T>) -> Result<&str, Error> {
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
