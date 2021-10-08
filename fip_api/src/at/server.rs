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

impl Server {
    /// TODO: documentation
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    /// TODO: documentation
    pub fn into_inner(self) -> InterceptedServer<AtServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
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
