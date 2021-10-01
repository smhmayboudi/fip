use crate::{
    at::{
        config::Config, controller::Controller, proto::server::at_server::AtServer,
        service::Service,
    },
    common::intercepted_server::InterceptedServer,
};

#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<AtServer<Controller>>,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    pub fn into_inner(self) -> InterceptedServer<AtServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
    fn default() -> Self {
        let config = Config::default();
        let service = Service::new(config.clone());
        let controller = Controller::new(config, service);

        Self {
            inner: InterceptedServer::new(AtServer::new(controller)),
        }
    }
}
