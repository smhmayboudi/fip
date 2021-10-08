// claims_sub => 1
// req.uri().path() => "fip.api.api.Api/FindOne"
// req.method() => "POST"

use casbin::{CoreApi, Enforcer, FileAdapter};

/// TODO: documentation
#[derive(Debug)]
pub struct AuthEnforce {}

impl AuthEnforce {
    /// TODO: documentation
    pub async fn enforce(sub: &str, obj: &str, act: &str) -> bool {
        let adapter = FileAdapter::new("auth_policy.csv");
        let enforcer = Enforcer::new("auth_model.conf", adapter)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
            })
            .unwrap();
        // Enforcer.set_logger(tracing);
        // Enforcer.enable_log(true);
        // let roles = enforcer.get_roles_for_user("alice");
        enforcer
            .enforce((sub, obj, act))
            .map_err(|err| {
                tracing::error!("{:?}", err);
            })
            .unwrap()
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
