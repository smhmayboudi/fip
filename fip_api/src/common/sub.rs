use fip_common::common_error::CommonError;
use tonic::Request;

pub trait Sub {
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
