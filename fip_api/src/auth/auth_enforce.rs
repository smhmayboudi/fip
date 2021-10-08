// claims_sub => 1
// req.uri().path() => "fip.api.api.Api/FindOne"
// req.method() => "POST"

use casbin::{CoreApi, Enforcer, FileAdapter};

/// TODO: documentation
#[derive(Copy, Clone, Debug)]
pub struct AuthEnforce {}

/// TODO: documentation
impl AuthEnforce {
    /// TODO: documentation
    ///
    /// # Panics
    /// TODO: documentation panics
    pub async fn enforce(sub: &str, obj: &str, act: &str) -> bool {
        let adapter = FileAdapter::new("fip_api/auth_policy.csv");
        let enforcer = Enforcer::new("fip_api/auth_model.conf", adapter)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
            })
            .unwrap_or_else(|err| {
                panic!("{:?}", err);
            });
        // Enforcer.set_logger(tracing);
        // Enforcer.enable_log(true);
        // let roles = enforcer.get_roles_for_user("alice");
        enforcer
            .enforce((sub, obj, act))
            .map_err(|err| {
                tracing::error!("{:?}", err);
            })
            .unwrap_or_else(|err| {
                panic!("{:?}", err);
            })
        // let sub = "alice"; // the user that wants to access a resource.
        // let obj = "data1"; // the resource that is going to be accessed.
        // let act = "read"; // the operation that the user performs on the resource.
        // if let Ok(authorized) = enforcer.enforce((sub, obj, act)) {
        //     if authorized {
        //         // permit alice to read data1
        //     } else {
        //         // deny the request
        //     }
        // } else {
        //     // error occurs
        // }
    }
}
