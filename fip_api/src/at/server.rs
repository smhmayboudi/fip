use crate::{
    at::{
        config::Config, controller::Controller, proto::server::at_server::AtServer,
        service::Service,
    },
    common::intercepted_server::InterceptedServer,
};

/// TODO: documentation
#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<AtServer<Controller>>,
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
    pub fn into_inner(self) -> InterceptedServer<AtServer<Controller>> {
        self.inner
    }
}

/// TODO: documentation
impl Default for Server {
    /// TODO: documentation
    fn default() -> Self {
        let config = Config::new();
        let service = Service::new(config.clone());
        let controller = Controller::new(config, service);

        let server = AtServer::new(controller);
        let intercepted_server = InterceptedServer::new(server);
        Self {
            inner: intercepted_server,
        }
    }
}
