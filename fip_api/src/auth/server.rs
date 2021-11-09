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

/// TODO: documentation
impl Server {
    /// TODO: documentation
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// TODO: documentation
impl Server {
    /// TODO: documentation
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn into_inner(self) -> InterceptedServer<AuthServer<Controller>> {
        self.inner
    }
}

/// TODO: documentation
impl Default for Server {
    /// TODO: documentation
    fn default() -> Self {
        let access_token_config = AtConfig::new();
        let access_token_service = AtService::new(access_token_config);
        let jwks_config = JwksConfig::new();
        let jwks_service = JwksService::new(jwks_config);
        let refresh_token_config = RtConfig::new();
        let refresh_token_service = RtService::new(refresh_token_config);
        let user_config = UserConfig::new();
        let user_service = UserService::new(user_config);
        let config = Config::new();
        let service = Service::new(
            config.clone(),
            access_token_service,
            jwks_service,
            refresh_token_service,
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
