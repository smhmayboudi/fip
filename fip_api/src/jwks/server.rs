use crate::{
    common::intercepted_server::InterceptedServer,
    jwks::{
        config::Config, controller::Controller, proto::server::jwks_server::JwksServer,
        service::Service,
    },
};

/// TODO: documentation
#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<JwksServer<Controller>>,
}

impl Server {
    /// TODO: documentation
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    /// TODO: documentation
    pub fn into_inner(self) -> InterceptedServer<JwksServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
    fn default() -> Self {
        let config = Config::new();
        let service = Service::new(config.clone());
        let controller = Controller::new(config, service);

        Self {
            inner: InterceptedServer::new(JwksServer::new(controller)),
        }
    }
}
