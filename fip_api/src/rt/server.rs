use crate::{
    common::intercepted_server::InterceptedServer,
    rt::{
        config::Config, controller::Controller, proto::server::rt_server::RtServer,
        service::Service,
    },
};

/// TODO: documentation
#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<RtServer<Controller>>,
}

impl Server {
    /// TODO: documentation
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    /// TODO: documentation
    pub fn into_inner(self) -> InterceptedServer<RtServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
    fn default() -> Self {
        let config = Config::new();
        let service = Service::new(config.clone());
        let controller = Controller::new(config, service);

        let server = RtServer::new(controller);
        let intercepted_server = InterceptedServer::new(server);
        Self {
            inner: intercepted_server,
        }
    }
}
