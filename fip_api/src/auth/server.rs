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

#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<AuthServer<Controller>>,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    pub fn into_inner(self) -> InterceptedServer<AuthServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
    fn default() -> Self {
        let at_config = AtConfig::default();
        let at_service = AtService::new(at_config);
        let jwks_config = JwksConfig::default();
        let jwks_service = JwksService::new(jwks_config);
        let rt_config = RtConfig::default();
        let rt_service = RtService::new(rt_config);
        let user_config = UserConfig::default();
        let user_service = UserService::new(user_config);
        let config = Config::default();
        let service = Service::new(
            config.clone(),
            at_service,
            jwks_service,
            rt_service,
            user_service,
        );
        let controller = Controller::new(config, service);

        Self {
            inner: InterceptedServer::new(AuthServer::new(controller)),
        }
    }
}
