use crate::{
    at::{config::Config as AtConfig, service::Service as AtService},
    auth::{
        config::Config, controller::Controller, proto::server::auth_server::AuthServer,
        service::Service,
    },
    common::intercepted_server::InterceptedServer,
    jwks::{config::Config as JwksConfig, service::Service as JwksService},
    rt::{config::Config as RtConfig, service::Service as RtService},
    user::{config::Config as UserConfig, service::Service as UserService},
};

/// TODO: documentation
#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<AuthServer<Controller>>,
}

impl Server {
    /// TODO: documentation
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    /// TODO: documentation
    pub fn into_inner(self) -> InterceptedServer<AuthServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
    fn default() -> Self {
        let at_config = AtConfig::new();
        let at_service = AtService::new(at_config);
        let jwks_config = JwksConfig::new();
        let jwks_service = JwksService::new(jwks_config);
        let rt_config = RtConfig::new();
        let rt_service = RtService::new(rt_config);
        let user_config = UserConfig::new();
        let user_service = UserService::new(user_config);
        let config = Config::new();
        let service = Service::new(
            config.clone(),
            at_service,
            jwks_service,
            rt_service,
            user_service,
        );
        let controller = Controller::new(config, service);

        let server = AuthServer::new(controller);
        let intercepted_server = InterceptedServer::new(server);
        Self {
            inner: intercepted_server,
        }
    }
}
