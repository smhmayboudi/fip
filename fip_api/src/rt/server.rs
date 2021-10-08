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
    pub fn into_inner(self) -> InterceptedServer<RtServer<Controller>> {
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

        let server = RtServer::new(controller);
        let intercepted_server = InterceptedServer::new(server);
        Self {
            inner: intercepted_server,
        }
    }
}
